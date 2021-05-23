use crate::config::CONFIG;
use crate::errors::ApiError;
use chrono::{DateTime, Utc};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::{SmtpTransport, Transport};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

pub fn send_mail(body_html: String, body_text: String, subject: String) -> Result<(), ApiError> {
    let to = &CONFIG.mail_receiver;
    let email = Message::builder()
        .from("新表单 <bflpsupp@163.com>".parse().unwrap())
        .reply_to("表单处理 <bflpsupp@163.com>".parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .multipart(
            MultiPart::alternative() // This is composed of two parts.
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType(
                            "text/plain; charset=utf8".parse().unwrap(),
                        ))
                        .body(body_text), // Every message should have a plain text fallback.
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType(
                            "text/html; charset=utf8".parse().unwrap(),
                        ))
                        .body(body_html),
                ),
        )?;

    let creds = Credentials::new(CONFIG.mail_user.clone(), CONFIG.mail_pwd.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_v) => println!("mail sent success"),
        Err(e) => println!("mail sent fail: {}", e),
    }
    Ok(())
}

pub fn ssmtp_mail(body_html: String, body_text: String, subject: String) -> Result<(), ApiError> {
    // create mail file
    let now: DateTime<Utc> = Utc::now();
    let file_path = format!("./data/mail/mail_{}.txt", now.timestamp_millis());
    let path = Path::new(&file_path);
    let display = path.display();

    let mail_tplt = format!(
        "to: {}
    MIME-Version: 1.0
    Content-Type: text/html; charset=utf-8
    Subject: {}
    
    {}",
        CONFIG.mail_receiver, subject, body_html
    );

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(mail_tplt.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }

    // start smtp shell ssmtp -t < testmail
    Command::new("ssmtp")
        .arg(format!("-t < {}", file_path))
        .spawn()
        .expect("ssmtp command failed to start");

    Ok(())
}
