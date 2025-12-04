use clap::Error as ClapError;
use lettre::address::AddressError;
use lettre::error::Error as LettreError;
use std::fmt;
use std::io::Error;

#[derive(Debug)]
pub enum AppError {
    Clap(String),
    Database(String),
    SmtpEmail(String),
    Config(String),
    IO(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Clap(e) => write!(f, "Argument error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::SmtpEmail(e) => write!(f, "Smtp error: {}", e),
            AppError::Config(e) => write!(f, "Config error: {}", e),
            AppError::IO(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl From<ClapError> for AppError {
    fn from(err: ClapError) -> Self {
        AppError::Clap(err.to_string())
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(value: toml::de::Error) -> Self {
        AppError::Config(value.to_string())
    }
}

impl From<Error> for AppError {
    fn from(value: Error) -> Self {
        AppError::IO(value.to_string())
    }
}

impl From<LettreError> for AppError {
    fn from(value: LettreError) -> Self {
        AppError::SmtpEmail(value.to_string())
    }
}

impl From<AddressError> for AppError {
    fn from(value: AddressError) -> Self {
        AppError::SmtpEmail(value.to_string())
    }
}
