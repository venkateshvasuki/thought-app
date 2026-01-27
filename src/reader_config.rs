use crate::errors::AppError;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,
    #[arg(short, long, default_value_t = true)]
    verbose: bool,
}
impl Args {
    pub fn parse_config(contents: &str) -> Result<Config, AppError> {
        let config: Config = toml::from_str(contents)?;
        Ok(config)
    }

    fn load_config(path: &PathBuf) -> Result<Config, AppError> {
        let contents = fs::read_to_string(path)?;
        Self::parse_config(&contents)
    }

    pub fn config(&self) -> Result<Config, AppError> {
        Self::load_config(&self.config)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailConfig {
    sender_email: String,
    receiver_email: String,
    app_password: String,
    relay: String,
    name: String,
}

impl EmailConfig {
    pub fn sender_email(&self) -> &String {
        &self.sender_email
    }
    pub fn receiver_email(&self) -> &String {
        &self.receiver_email
    }
    pub fn app_password(&self) -> &String {
        &self.app_password
    }
    pub fn relay(&self) -> &String {
        &self.relay
    }
    pub fn name(&self) -> &String {
        &self.name
    }
}
pub trait AIClientDetails {
    fn endpoint(&self) -> &str;
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AIClient {
    OpenAI,
    Claude,
    Gemini,
}
impl AIClientDetails for AIClient {
    fn endpoint(&self) -> &str {
        match self {
            AIClient::OpenAI => "https://api.openai.com/v1/chat/completions",
            AIClient::Gemini => {
                "https://generativelanguage.googleapis.com/v1/models/gemini-2.0-flash:generateContent"
            }
            AIClient::Claude => "https://api.anthropic.com/v1/messages",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AIClientConfig {
    bearer_token: String,
    ai_client: AIClient,
}

impl Clone for AIClient {
    fn clone(&self) -> Self {
        match self {
            AIClient::Claude => AIClient::Claude,
            AIClient::OpenAI => AIClient::OpenAI,
            AIClient::Gemini => AIClient::Gemini,
        }
    }
}

impl AIClientConfig {
    pub fn bearer_token(&self) -> &String {
        &self.bearer_token
    }
    pub fn ai_client(&self) -> &AIClient {
        &self.ai_client
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    ai_client_config: AIClientConfig,
    email_config: EmailConfig,
}

impl Config {
    pub fn ai_client_config(&self) -> AIClientConfig {
        let bearer_token = match self.ai_client_config.ai_client {
            AIClient::Claude => self.ai_client_config.bearer_token.clone(),
            _ => format!("Bearer {}", self.ai_client_config.bearer_token.clone()),
        };

        AIClientConfig {
            ai_client: self.ai_client_config.ai_client.clone(),
            bearer_token,
        }
    }

    pub fn email_config(&self) -> &EmailConfig {
        &self.email_config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_config_valid_toml() {
        let toml_content = r#"
            [ai_client_config]
            bearer_token = "test_token"
            ai_client = "OpenAI"

            [email_config]
            sender_email = "sender@example.com"
            receiver_email = "receiver@example.com"
            app_password = "password123"
            relay = "smtp.example.com"
            name = "Test User"
        "#;

        let config = Args::parse_config(toml_content).unwrap();
        assert_eq!(config.ai_client_config.bearer_token(), "test_token");
        assert_eq!(config.email_config().sender_email(), "sender@example.com");
    }

    #[test]
    fn test_parse_config_invalid_toml() {
        let invalid_toml = "invalid toml content [[[";
        let result = Args::parse_config(invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_config_missing_fields() {
        let incomplete_toml = r#"
            [ai_client_config]
            bearer_token = "test_token"
        "#;
        let result = Args::parse_config(incomplete_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_config_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        let toml_content = r#"
            [ai_client_config]
            bearer_token = "file_token"
            ai_client = "Claude"

            [email_config]
            sender_email = "test@example.com"
            receiver_email = "dest@example.com"
            app_password = "pass"
            relay = "smtp.test.com"
            name = "Tester"
        "#;
        temp_file.write_all(toml_content.as_bytes()).unwrap();

        let args = Args {
            config: temp_file.path().to_path_buf(),
            verbose: true,
        };

        let config = args.config().unwrap();
        assert_eq!(config.ai_client_config.bearer_token(), "file_token");
    }

    #[test]
    fn test_load_config_file_not_found() {
        let args = Args {
            config: PathBuf::from("/nonexistent/path/config.toml"),
            verbose: true,
        };

        let result = args.config();
        assert!(result.is_err());
    }

    #[test]
    fn test_bearer_token_formatting_claude() {
        let toml_content = r#"
            [ai_client_config]
            bearer_token = "claude_key"
            ai_client = "Claude"

            [email_config]
            sender_email = "test@test.com"
            receiver_email = "test@test.com"
            app_password = "pass"
            relay = "smtp.test.com"
            name = "Test"
        "#;

        let config = Args::parse_config(toml_content).unwrap();
        let ai_config = config.ai_client_config();
        assert_eq!(ai_config.bearer_token(), "claude_key");
    }

    #[test]
    fn test_bearer_token_formatting_openai() {
        let toml_content = r#"
            [ai_client_config]
            bearer_token = "openai_key"
            ai_client = "OpenAI"

            [email_config]
            sender_email = "test@test.com"
            receiver_email = "test@test.com"
            app_password = "pass"
            relay = "smtp.test.com"
            name = "Test"
        "#;

        let config = Args::parse_config(toml_content).unwrap();
        let ai_config = config.ai_client_config();
        assert_eq!(ai_config.bearer_token(), "Bearer openai_key");
    }

    #[test]
    fn test_bearer_token_formatting_gemini() {
        let toml_content = r#"
            [ai_client_config]
            bearer_token = "gemini_key"
            ai_client = "Gemini"

            [email_config]
            sender_email = "test@test.com"
            receiver_email = "test@test.com"
            app_password = "pass"
            relay = "smtp.test.com"
            name = "Test"
        "#;

        let config = Args::parse_config(toml_content).unwrap();
        let ai_config = config.ai_client_config();
        assert_eq!(ai_config.bearer_token(), "Bearer gemini_key");
    }

    #[test]
    fn test_email_config_getters() {
        let toml_content = r#"
            [ai_client_config]
            bearer_token = "token"
            ai_client = "OpenAI"

            [email_config]
            sender_email = "sender@test.com"
            receiver_email = "receiver@test.com"
            app_password = "secret123"
            relay = "smtp.gmail.com"
            name = "John Doe"
        "#;

        let config = Args::parse_config(toml_content).unwrap();
        let email = config.email_config();
        assert_eq!(email.sender_email(), "sender@test.com");
        assert_eq!(email.receiver_email(), "receiver@test.com");
        assert_eq!(email.app_password(), "secret123");
        assert_eq!(email.relay(), "smtp.gmail.com");
        assert_eq!(email.name(), "John Doe");
    }
}
