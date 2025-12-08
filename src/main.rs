mod client;
mod db_operations;
mod email;
mod errors;
mod reader_config;
mod thought;
mod writer_config;
use crate::db_operations::{read, setup_db, write_to_db};
use crate::reader_config::Args as ReaderConfigArgs;
use crate::writer_config::{Args as WriterConfigArgs, ThoughtType};
use clap::Parser;
use std::env;

fn get_db_path() -> String {
    env::var("DB_PATH").unwrap_or_else(|_| "thought_app.db".to_string())
}
#[cfg(feature = "writer")]
fn main() -> Result<(), errors::AppError> {
    let args = WriterConfigArgs::try_parse()?;
    let conn = setup_db(&get_db_path())?;
    write_to_db(&conn, &args)
}

//#[cfg(feature = "reader")]
fn main() -> Result<(), errors::AppError> {
    let args = ReaderConfigArgs::try_parse()?;
    let config = args.config()?;
    let conn = setup_db(&get_db_path())?;
    let thoughts = read(&conn)?;
    let content: Vec<&String> = thoughts
        .iter()
        .filter(|s| matches!(s.thought_type(), ThoughtType::Project))
        .map(|s| s.content())
        .collect();

    let results = client::get_response(&config.ai_client_config(), &content)?;
    println!("Results {results}");
    email::send_email(&thoughts, &config.email_config())?;
    Ok(())
}
