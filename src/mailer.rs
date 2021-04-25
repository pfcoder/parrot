use lettre::message::{header, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use std::env;

pub fn send_mail(body_html: String, body_text: String, subject: String) {
	let email = Message::builder()
		.from("test <bflpsupp@163.com>".parse().unwrap())
		.reply_to("test <bflpsupp@163.com>".parse().unwrap())
		.to("report <3276628@qq.com>".parse().unwrap())
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

	let creds = Credentials::new(
		env::var("MAIL_USERNAME").unwrap_or("user".to_string()),
		env::var("MAIL_PASSWORD").unwrap_or("password".to_string()),
	);

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
}
