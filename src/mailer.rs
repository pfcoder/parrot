use crate::config::CONFIG;
use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
//use lettre::{AsyncSmtpTransport, Message, Tokio02Executor};
use lettre::AsyncTransport;
use lettre::{AsyncSmtpTransport, Message, Tokio02Executor};
use lettre::{SmtpTransport, Transport};

//use std::env;
//#[tokio::main]
pub fn send_mail(body_html: String, body_text: String, subject: String) {
    let to = &CONFIG.mail_receiver;
    let email = Message::builder()
        .from("test <bflpsupp@163.com>".parse().unwrap())
        .reply_to("test <bflpsupp@163.com>".parse().unwrap())
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
        )
        .unwrap();

    let creds = Credentials::new(CONFIG.mail_user.clone(), CONFIG.mail_pwd.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    /*let mailer: AsyncSmtpTransport<Tokio02Executor> =
        AsyncSmtpTransport::<Tokio02Executor>::relay("smtp.163.com")
            .unwrap()
            .credentials(creds)
            .build();

    match mailer.send(email).await {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => println!("Could not send email: {:?}", e),
    }*/
}

/*pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

use async_smtp::{ClientSecurity, Envelope, SendableEmail, SmtpClient, Transport};

async fn send_mail(body_html: String, body_text: String, subject: String) -> Result<()> {
    let email = SendableEmail::new(
        Envelope::new(
            Some("user@localhost".parse().unwrap()),
            vec![CONFIG.mail_receiver.parse().unwrap()],
        )?,
        "id",
        "Hello world",
    );

    // Create a client
    let mut smtp = SmtpClient::new("127.0.0.1:2525").await?.into_transport();

    // Connect and send the email.
    smtp.send(email).await?;

    Ok(())
}*/
