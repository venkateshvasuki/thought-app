use clap::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Clap(Error),
    Database(rusqlite::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Clap(e) => write!(f, "Argument error: {}", e),
            AppError::Database(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl From<Error> for AppError {
    fn from(err: Error) -> Self {
        AppError::Clap(err)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err)
    }
}
