use crate::errors::AppError;
use crate::thought::Thought;
use crate::writer_config::Args;
use rusqlite::{Connection, Result as SqlResult};

fn read_from_db(conn: &Connection) -> Result<Vec<Thought>, AppError> {
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
    thoughts.iter().for_each(|t| {
        let _ = conn.execute("UPDATE thoughts SET reviewed = true WHERE id = ?", [t.id()]);
    });
    Ok(thoughts)
}

fn update_db(conn: &Connection, ids: &[Thought]) -> Result<(), AppError> {
    ids.iter().try_for_each(|thought| {
        conn.execute(
            "UPDATE thoughts SET reviewed = true WHERE id = ?",
            [thought.id()],
        )?;
        Ok(())
    })
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

pub fn read(conn: &Connection) -> Result<Vec<Thought>, AppError> {
    let thoughts = read_from_db(conn)?;
    update_db(conn, &thoughts)?;
    Ok(thoughts)
}
pub fn write_to_db(conn: &Connection, args: &Args) -> SqlResult<(), AppError> {
    conn.execute(
        "INSERT INTO thoughts (type, content) VALUES (?, ?)",
        [args.thought_type().to_string(), args.content().to_string()],
    )?;
    Ok(())
}
