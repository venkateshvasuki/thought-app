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
    fn load_config(path: &PathBuf) -> Result<Config, AppError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
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
trait AIClientDetails {
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
            AIClient::Gemini => "https://generativelanguage.googleapis.com/v1/models",
            AIClient::Claude => "https://api.anthropic.com/v1/messages",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIClientConfig {
    bearer_token: String,
    ai_client: AIClient,
}

impl AIClientConfig {
    pub fn bearer_token(&self) -> String {
        match self.ai_client {
            AIClient::Claude => self.bearer_token.clone(),
            _ => format!("Bearer {}", self.bearer_token),
        }
    }
    pub fn endpoint(&self) -> &str {
        &self.ai_client.endpoint()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    ai_client_config: AIClientConfig,
    email_config: EmailConfig,
}

impl Config {
    pub fn ai_client_config(&self) -> &AIClientConfig {
        &self.ai_client_config
    }
    pub fn email_config(&self) -> &EmailConfig {
        &self.email_config
    }
}
