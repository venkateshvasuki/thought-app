use crate::config::Args;
use crate::errors::AppError;
use crate::thought::Thought;
use rusqlite::{Connection, Result as SqlResult};

fn write_to_db(conn: &Connection, args: &Args) -> SqlResult<(), AppError> {
    conn.execute(
        "INSERT INTO thoughts (type, content) VALUES (?, ?)",
        [args.thought_type().to_string(), args.content().to_string()],
    )?;
    Ok(())
}

fn read_from_db(conn: &Connection) -> SqlResult<Vec<Thought>> {
    let thoughts: Vec<Thought> = conn
        .prepare("SELECT id, type, content, reviewed FROM thoughts WHERE reviewed = false")?
        .query_map([], |row| {
            let id = row.get(0)?;
            let thought_type = row.get(1)?;
            let content = row.get(2)?;
            let reviewed = row.get(3)?;
            Ok(Thought::new(id, thought_type, content, reviewed))
        })?
        .collect::<SqlResult<Vec<Thought>>>()?;
    Ok(thoughts)
}

pub fn setup_db(db_name: &str) -> SqlResult<Connection, AppError> {
    let conn = Connection::open(db_name)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS thoughts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            type TEXT NOT NULL,
            content TEXT NOT NULL,
            reviewed BOOLEAN NOT NULL DEFAULT FALSE
        )",
        [],
    )?;
    Ok(conn)
}

pub fn execute(conn: &Connection, args: &Args) -> Result<(), AppError> {
    write_to_db(&conn, &args)?;
    let res = read_from_db(&conn)?;
    for thought in &res {
        conn.execute(
            "UPDATE thoughts SET reviewed = true WHERE id = ?",
            [&thought.id()],
        )?;
    }
    println!("{:?}", res);
    Ok(())
}
