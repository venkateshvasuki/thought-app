use clap::{Parser, ValueEnum};
use rusqlite::types::{FromSql, FromSqlError};
use strum_macros::{Display, EnumString};
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, value_enum)]
    thought_type: ThoughtType,
    #[arg(short = 'c', long)]
    content: String,
}

impl Args {
    pub fn thought_type(&self) -> &ThoughtType {
        &self.thought_type
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[derive(Display, Debug, Clone, ValueEnum, EnumString)]
pub enum ThoughtType {
    Notes,
    Project,
    Misc,
    Todo,
    Question,
}

use std::io::{Error, ErrorKind};

impl ThoughtType {
    pub fn from_str(string: &str) -> Result<ThoughtType, Error> {
        match string.to_lowercase().as_str() {
            "notes" => Ok(ThoughtType::Notes),
            "project" => Ok(ThoughtType::Project),
            "misc" => Ok(ThoughtType::Misc),
            "todo" => Ok(ThoughtType::Todo),
            "question" => Ok(ThoughtType::Question),
            s => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unknown thought type: {s}"),
            )),
        }
    }
}

impl FromSql for ThoughtType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let string = value.as_str()?;
        Self::from_str(string)
            .map_err(|e| FromSqlError::Other(Box::new(Error::new(ErrorKind::InvalidData, e))))
    }
}
