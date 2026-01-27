#[cfg(feature = "writer")]
mod writer_tests {
    use thought::db_operations::{setup_db, write_to_db};
    use thought::writer_config::{Args, ThoughtType};

    #[test]
    fn test_write_and_verify_end_to_end() {
        // Create in-memory database
        let conn = setup_db(":memory:").unwrap();

        // Create args and write a thought
        let args = Args::new_for_test(
            ThoughtType::Notes,
            "Integration test thought".to_string(),
        );

        write_to_db(&conn, &args).unwrap();

        // Verify the thought was written
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM thoughts", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);

        // Verify the content
        let content: String = conn
            .query_row(
                "SELECT content FROM thoughts WHERE id = 1",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(content, "Integration test thought");
    }

    #[test]
    fn test_write_multiple_thoughts() {
        let conn = setup_db(":memory:").unwrap();

        // Write multiple thoughts
        let thoughts = vec![
            Args::new_for_test(ThoughtType::Notes, "First thought".to_string()),
            Args::new_for_test(ThoughtType::Project, "Second thought".to_string()),
            Args::new_for_test(ThoughtType::Todo, "Third thought".to_string()),
            Args::new_for_test(ThoughtType::Question, "Fourth thought".to_string()),
            Args::new_for_test(ThoughtType::Misc, "Fifth thought".to_string()),
        ];

        for args in &thoughts {
            write_to_db(&conn, args).unwrap();
        }

        // Verify count
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM thoughts", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 5);

        // Verify all are unreviewed
        let unreviewed: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM thoughts WHERE reviewed = false",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert_eq!(unreviewed, 5);
    }

    #[test]
    fn test_write_different_thought_types() {
        let conn = setup_db(":memory:").unwrap();

        let thought_types = vec![
            ThoughtType::Notes,
            ThoughtType::Project,
            ThoughtType::Todo,
            ThoughtType::Question,
            ThoughtType::Misc,
        ];

        for (i, thought_type) in thought_types.iter().enumerate() {
            let args = Args::new_for_test(thought_type.clone(), format!("Thought {}", i));
            write_to_db(&conn, &args).unwrap();
        }

        // Verify each type was written
        for thought_type in thought_types {
            let count: i32 = conn
                .query_row(
                    "SELECT COUNT(*) FROM thoughts WHERE type = ?",
                    [thought_type.to_string()],
                    |row| row.get(0),
                )
                .unwrap();

            assert_eq!(count, 1);
        }
    }
}
