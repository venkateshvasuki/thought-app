use clap::{Parser, ValueEnum};
use rusqlite::types::{FromSql, FromSqlError};
use std::str::FromStr;
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
#[strum(ascii_case_insensitive)]
pub enum ThoughtType {
    Notes,
    Project,
    Misc,
    Todo,
    Question,
}
impl FromSql for ThoughtType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let string = value.as_str()?;
        <ThoughtType as FromStr>::from_str(string).map_err(|e| {
            FromSqlError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )))
        })
    }
}
