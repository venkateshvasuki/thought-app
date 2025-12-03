mod config;
mod db_operations;
mod errors;
mod thought;

use clap::Parser;

use crate::config::Args;
use crate::db_operations::{read, setup_db, write_to_db};

#[cfg(feature = "writer")]
fn main() -> Result<(), errors::AppError> {
    let args = Args::try_parse()?;
    let conn = setup_db("my_database.db")?;
    write_to_db(&conn, &args)
}

#[cfg(feature = "reader")]
fn main() -> Result<(), errors::AppError> {
    let conn = setup_db("my_database.db")?;
    let res = read(&conn)?;
    res.iter().for_each(|thought| println!("{:?}", thought));
    Ok(())
}
