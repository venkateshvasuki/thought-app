use crate::errors::AppError;
use crate::errors::AppError::SmtpEmail;
use crate::reader_config::Config;
use crate::thought::{Thought, ThoughtsEmailBody};
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
pub fn send_email(thought: &[Thought], config: &Config) -> Result<(), AppError> {
    let email = Message::builder()
        .from(Mailbox::new(
            Some("Thought App".to_string()),
            config.sender_email().parse()?,
        ))
        .to(Mailbox::new(
            Some(config.name().to_string()),
            config.receiver_email().parse()?,
        ))
        .subject("Thought App, Weekly Round up")
        .header(ContentType::TEXT_PLAIN)
        .body(ThoughtsEmailBody::new(thought))?;

    let creds = Credentials::new(
        config.sender_email().to_string(),
        config.app_password().to_string(),
    );

    let mailer = SmtpTransport::starttls_relay(config.relay())
        .map_err(|e| SmtpEmail(e.to_string()))?
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map(|_| ())
        .map_err(|e| SmtpEmail(e.to_string()))
}
