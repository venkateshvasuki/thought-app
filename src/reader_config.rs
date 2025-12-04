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
pub struct Config {
    sender_email: String,
    receiver_email: String,
    app_password: String,
    relay: String,
    name: String,
}

impl Config {
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
