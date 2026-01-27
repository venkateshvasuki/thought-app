#[cfg(feature = "reader")]
mod reader_tests {
    use thought::db_operations::{read, setup_db};
    use thought::writer_config::ThoughtType;

    #[test]
    fn test_read_end_to_end() {
        let conn = setup_db(":memory:").unwrap();

        // Populate database with test data - use 0 for false in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Notes", "First unreviewed", "0"],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Todo", "Second unreviewed", "0"],
        )
        .unwrap();

        // Read thoughts
        let thoughts = read(&conn).unwrap();

        // Verify thoughts were returned
        assert_eq!(thoughts.len(), 2);

        // Verify they are now marked as reviewed - use 0 and 1 for SQLite booleans
        let unreviewed: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM thoughts WHERE reviewed = 0",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(unreviewed, 0);

        let reviewed: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM thoughts WHERE reviewed = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(reviewed, 2);
    }

    #[test]
    fn test_read_twice_returns_empty() {
        let conn = setup_db(":memory:").unwrap();

        // Add test thought - use 0 for false in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Notes", "Test thought", "0"],
        )
        .unwrap();

        // First read should return thought
        let first_read = read(&conn).unwrap();
        assert_eq!(first_read.len(), 1);

        // Second read should return empty
        let second_read = read(&conn).unwrap();
        assert_eq!(second_read.len(), 0);
    }

    #[test]
    fn test_read_ignores_already_reviewed() {
        let conn = setup_db(":memory:").unwrap();

        // Add reviewed thought - use 1 for true in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Notes", "Already reviewed", "1"],
        )
        .unwrap();

        // Add unreviewed thought - use 0 for false in SQLite
        conn.execute(
            "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
            ["Todo", "Not reviewed", "0"],
        )
        .unwrap();

        let thoughts = read(&conn).unwrap();

        // Should only return unreviewed thought
        assert_eq!(thoughts.len(), 1);
        assert_eq!(thoughts[0].content(), "Not reviewed");
    }

    #[test]
    fn test_read_empty_database() {
        let conn = setup_db(":memory:").unwrap();

        let thoughts = read(&conn).unwrap();
        assert_eq!(thoughts.len(), 0);
    }

    #[test]
    fn test_read_multiple_thought_types() {
        let conn = setup_db(":memory:").unwrap();

        // Add various thought types - use 0 for false in SQLite
        let types = vec!["Notes", "Project", "Todo", "Question", "Misc"];
        for thought_type in &types {
            conn.execute(
                "INSERT INTO thoughts (type, content, reviewed) VALUES (?, ?, ?)",
                [*thought_type, &format!("{} content", thought_type), "0"],
            )
            .unwrap();
        }

        let thoughts = read(&conn).unwrap();

        // Verify all types were read
        assert_eq!(thoughts.len(), 5);

        // Verify types match
        let mut read_types: Vec<String> = thoughts
            .iter()
            .map(|t| t.thought_type().to_string())
            .collect();
        read_types.sort();

        let mut expected_types: Vec<String> =
            types.iter().map(|s| s.to_string()).collect();
        expected_types.sort();

        assert_eq!(read_types, expected_types);
    }
}
