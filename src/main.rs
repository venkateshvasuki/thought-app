mod config;
mod db_operations;
mod errors;
mod thought;

use clap::Parser;

use crate::config::Args;
use crate::db_operations::{execute, setup_db};

fn main() -> Result<(), errors::AppError> {
    let args = Args::try_parse()?;
    let conn = setup_db("my_database.db")?;
    execute(&conn, &args)
}
