mod config;
mod db_operations;
mod errors;
mod thought;

use crate::config::Args;
use crate::db_operations::{read, setup_db, write_to_db};
use clap::Parser;
use std::env;

fn get_db_path() -> String {
    env::var("DB_PATH").unwrap_or_else(|_| "thought_app.db".to_string())
}
#[cfg(feature = "writer")]
fn main() -> Result<(), errors::AppError> {
    let args = Args::try_parse()?;
    let conn = setup_db(&get_db_path())?;
    write_to_db(&conn, &args)
}

#[cfg(feature = "reader")]
fn main() -> Result<(), errors::AppError> {
    let conn = setup_db(&get_db_path())?;
    let res = read(&conn)?;
    res.iter().for_each(|thought| println!("{:?}", thought));
    Ok(())
}
