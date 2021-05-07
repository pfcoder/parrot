use crate::config::CONFIG;
use crate::errors::ApiError;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::Message;
use lettre::{SmtpTransport, Transport};

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
