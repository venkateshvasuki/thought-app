use crate::config::ThoughtType;
use crate::thought::Thought;
use rusqlite::{Connection, Result as SqlResult};

pub fn write_to_db(conn: &Connection, thought_type: &ThoughtType, content: &str) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO thoughts (type, content) VALUES (?, ?)",
        [thought_type.to_string(), content.to_string()],
    )?;
    Ok(())
}

pub fn read_from_db(conn: &Connection) -> SqlResult<Vec<Thought>> {
    conn.prepare("SELECT id, type, content, reviewed FROM thoughts")
        .and_then(|mut res| {
            res.query_and_then([], |row| {
                let id = row.get(0)?;
                let thought_type = row.get(1)?;
                let content = row.get(2)?;
                let reviewed = row.get(3)?;
                Ok(Thought {
                    id,
                    thought_type,
                    content,
                    reviewed,
                })
            })?
            .collect()
        })
}
pub fn setup_db(db_name: &str) -> SqlResult<Connection> {
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
