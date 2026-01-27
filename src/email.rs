use crate::errors::AppError;
use crate::errors::AppError::SmtpEmail;
use crate::reader_config::{Config, EmailConfig};
use crate::thought::{Thought, ThoughtsEmailBody};
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub trait EmailTransport {
    fn send(&self, email: &Message) -> Result<(), String>;
}

impl EmailTransport for SmtpTransport {
    fn send(&self, email: &Message) -> Result<(), String> {
        Transport::send(self, email)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

pub fn send_email_with_transport<T: EmailTransport>(
    thought: &[Thought],
    config: &EmailConfig,
    transport: &T,
) -> Result<(), AppError> {
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
        .header(ContentType::TEXT_HTML)
        .body(ThoughtsEmailBody::new(thought))?;

    transport.send(&email).map_err(|e| AppError::SmtpEmail(e))
}

pub fn send_email(thought: &[Thought], config: &EmailConfig) -> Result<(), AppError> {
    let creds = Credentials::new(
        config.sender_email().to_string(),
        config.app_password().to_string(),
    );

    let mailer = SmtpTransport::starttls_relay(config.relay())
        .map_err(|e| SmtpEmail(e.to_string()))?
        .credentials(creds)
        .build();

    send_email_with_transport(thought, config, &mailer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer_config::ThoughtType;
    use std::cell::RefCell;

    struct MockEmailTransport {
        should_fail: bool,
        sent_emails: RefCell<Vec<String>>,
    }

    impl EmailTransport for MockEmailTransport {
        fn send(&self, _email: &Message) -> Result<(), String> {
            if self.should_fail {
                Err("Mock SMTP error".to_string())
            } else {
                self.sent_emails.borrow_mut().push("sent".to_string());
                Ok(())
            }
        }
    }

    fn create_test_email_config() -> EmailConfig {
        serde_json::from_str(
            r#"{
            "sender_email": "test@example.com",
            "receiver_email": "receiver@example.com",
            "app_password": "password",
            "relay": "smtp.example.com",
            "name": "Test User"
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_send_email_success() {
        let config = create_test_email_config();
        let thoughts = vec![Thought::new(
            1,
            ThoughtType::Notes,
            "Test thought".to_string(),
            false,
        )];

        let transport = MockEmailTransport {
            should_fail: false,
            sent_emails: RefCell::new(Vec::new()),
        };

        let result = send_email_with_transport(&thoughts, &config, &transport);
        assert!(result.is_ok());
        assert_eq!(transport.sent_emails.borrow().len(), 1);
    }

    #[test]
    fn test_send_email_failure() {
        let config = create_test_email_config();
        let thoughts = vec![Thought::new(
            1,
            ThoughtType::Todo,
            "Test".to_string(),
            false,
        )];

        let transport = MockEmailTransport {
            should_fail: true,
            sent_emails: RefCell::new(Vec::new()),
        };

        let result = send_email_with_transport(&thoughts, &config, &transport);
        assert!(result.is_err());
        assert_eq!(transport.sent_emails.borrow().len(), 0);
    }

    #[test]
    fn test_send_email_empty_thoughts() {
        let config = create_test_email_config();
        let thoughts: Vec<Thought> = vec![];

        let transport = MockEmailTransport {
            should_fail: false,
            sent_emails: RefCell::new(Vec::new()),
        };

        let result = send_email_with_transport(&thoughts, &config, &transport);
        assert!(result.is_ok());
        assert_eq!(transport.sent_emails.borrow().len(), 1);
    }

    #[test]
    fn test_send_email_multiple_thoughts() {
        let config = create_test_email_config();
        let thoughts = vec![
            Thought::new(1, ThoughtType::Notes, "First".to_string(), false),
            Thought::new(2, ThoughtType::Project, "Second".to_string(), false),
            Thought::new(3, ThoughtType::Question, "Third".to_string(), false),
        ];

        let transport = MockEmailTransport {
            should_fail: false,
            sent_emails: RefCell::new(Vec::new()),
        };

        let result = send_email_with_transport(&thoughts, &config, &transport);
        assert!(result.is_ok());
        assert_eq!(transport.sent_emails.borrow().len(), 1);
    }

    #[test]
    fn test_send_email_invalid_email_address() {
        let mut config = create_test_email_config();
        // Create invalid config using JSON manipulation
        let invalid_config_json = r#"{
            "sender_email": "invalid-email",
            "receiver_email": "receiver@example.com",
            "app_password": "password",
            "relay": "smtp.example.com",
            "name": "Test User"
        }"#;

        let invalid_config: EmailConfig = serde_json::from_str(invalid_config_json).unwrap();
        let thoughts = vec![Thought::new(
            1,
            ThoughtType::Notes,
            "Test".to_string(),
            false,
        )];

        let transport = MockEmailTransport {
            should_fail: false,
            sent_emails: RefCell::new(Vec::new()),
        };

        let result = send_email_with_transport(&thoughts, &invalid_config, &transport);
        assert!(result.is_err());
    }
}
