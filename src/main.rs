mod config;
mod db_operations;
mod errors;
mod thought;

use clap::Parser;

use crate::config::Args;
use crate::db_operations::{read_from_db, setup_db, write_to_db};

fn main() -> Result<(), errors::AppError> {
    let args = Args::try_parse()?;
    let conn = setup_db("my_database.db")?;
    write_to_db(&conn, &args.thought_type, &args.content)?;
    let res = read_from_db(&conn)?;
    println!("{:#?}", res);
    Ok(())
}
