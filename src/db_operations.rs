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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer_config::ThoughtType;

    fn create_in_memory_db() -> Connection {
        setup_db(":memory:").unwrap()
    }

    #[test]
    fn test_setup_db_creates_table() {
        let conn = create_in_memory_db();

        // Verify table exists by querying it
        let result: Result<i32, _> = conn.query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='thoughts'",
            [],
            |row| row.get(0),
        );

        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_write_to_db_inserts_thought() {
        let conn = create_in_memory_db();
        let args = Args::new_for_test(ThoughtType::Notes, "Test thought".to_string());

        write_to_db(&conn, &args).unwrap();

        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM thoughts", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
    }

    #[test]
    fn test_read_from_db_returns_unreviewed_thoughts() {
        let conn = create_in_memory_db();

        // Insert test data - use 0 and 1 for boolean values in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Notes", "Unreviewed thought", "0"],
        ).unwrap();
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Todo", "Reviewed thought", "1"],
        ).unwrap();

        let thoughts = read_from_db(&conn).unwrap();

        // read_from_db returns thoughts that were unreviewed, and marks them as reviewed
        assert_eq!(thoughts.len(), 1);
        assert_eq!(thoughts[0].content(), "Unreviewed thought");
        // The thought struct still has reviewed: false as it was when fetched
        assert_eq!(thoughts[0].reviewed(), false);

        // But in DB it should now be marked as reviewed
        let reviewed_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM thoughts WHERE reviewed = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(reviewed_count, 2); // Both thoughts are now reviewed
    }

    #[test]
    fn test_read_marks_thoughts_as_reviewed() {
        let conn = create_in_memory_db();

        // Insert unreviewed thought - use 0 for false in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Notes", "Test thought", "0"],
        ).unwrap();

        let thoughts = read(&conn).unwrap();
        assert_eq!(thoughts.len(), 1);

        // Second read should return empty
        let thoughts_second = read(&conn).unwrap();
        assert_eq!(thoughts_second.len(), 0);
    }

    #[test]
    fn test_update_db_updates_multiple_thoughts() {
        let conn = create_in_memory_db();

        // Insert multiple unreviewed thoughts - use 0 for false in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Notes", "First", "0"],
        ).unwrap();
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Todo", "Second", "0"],
        ).unwrap();

        // Note: read_from_db already marks thoughts as reviewed
        // So to test update_db separately, we need to use a different approach
        // Let's query thoughts without the update
        let thoughts: Vec<Thought> = conn
            .prepare("SELECT id, type, content, reviewed FROM thoughts WHERE reviewed = 0")
            .unwrap()
            .query_map([], |row| {
                let id = row.get(0)?;
                let thought_type = row.get(1)?;
                let content = row.get(2)?;
                let reviewed = row.get(3)?;
                Ok(Thought::new(id, thought_type, content, reviewed))
            })
            .unwrap()
            .collect::<SqlResult<Vec<Thought>>>()
            .unwrap();

        assert_eq!(thoughts.len(), 2);

        update_db(&conn, &thoughts).unwrap();

        // Verify all are marked as reviewed - use 1 for true in SQLite
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM thoughts WHERE reviewed = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(count, 2);
    }

    #[test]
    fn test_empty_database_read() {
        let conn = create_in_memory_db();

        let thoughts = read(&conn).unwrap();
        assert_eq!(thoughts.len(), 0);
    }
}
