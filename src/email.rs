use crate::errors::AppError;
use crate::errors::AppError::Smtp;
use crate::reader_config::Config;
use crate::thought::Thought;
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(thought: &[Thought], config: &Config) -> Result<(), AppError> {
    let email = Message::builder()
        .from(Mailbox::new(
            Some("Thought App".to_owned()),
            config.sender_email().parse().unwrap(),
        ))
        .to(Mailbox::new(
            config.name().to_owned(),
            config.receiver_email().parse().unwrap(),
        ))
        .subject("Thought App, Weekly Round up")
        .header(ContentType::TEXT_PLAIN)
        .body(format!("Body: {:?}", thought))
        .map_err(|e| Smtp(e.to_string()))?;

    let creds = Credentials::new(
        config.sender_email().to_owned(),
        config.app_password().to_owned(),
    );

    let mailer = SmtpTransport::starttls_relay(config.relay())
        .map_err(|e| Smtp(e.to_string()))?
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map(|_| ())
        .map_err(|e| Smtp(e.to_string()))
}
