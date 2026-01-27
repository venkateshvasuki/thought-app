use clap::{Parser, ValueEnum};
use rusqlite::types::{FromSql, FromSqlError};
use std::str::FromStr;
use strum_macros::{Display, EnumString};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, value_enum)]
    thought_type: ThoughtType,
    #[arg(short = 'c', long)]
    content: String,
}

impl Args {
    pub fn thought_type(&self) -> &ThoughtType {
        &self.thought_type
    }
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Create Args for testing purposes
    /// Only available in test builds
    pub fn new_for_test(thought_type: ThoughtType, content: String) -> Self {
        Args {
            thought_type,
            content,
        }
    }
}

#[derive(Display, Debug, Clone, ValueEnum, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ThoughtType {
    Notes,
    Project,
    Misc,
    Todo,
    Question,
}
impl FromSql for ThoughtType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let string = value.as_str()?;
        <ThoughtType as FromStr>::from_str(string).map_err(|e| {
            FromSqlError::Other(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::types::ValueRef;

    #[test]
    fn test_thought_type_display() {
        assert_eq!(ThoughtType::Notes.to_string(), "Notes");
        assert_eq!(ThoughtType::Project.to_string(), "Project");
        assert_eq!(ThoughtType::Misc.to_string(), "Misc");
        assert_eq!(ThoughtType::Todo.to_string(), "Todo");
        assert_eq!(ThoughtType::Question.to_string(), "Question");
    }

    #[test]
    fn test_thought_type_from_str() {
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("Notes").unwrap(),
            ThoughtType::Notes
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("Project").unwrap(),
            ThoughtType::Project
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("Misc").unwrap(),
            ThoughtType::Misc
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("Todo").unwrap(),
            ThoughtType::Todo
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("Question").unwrap(),
            ThoughtType::Question
        ));
    }

    #[test]
    fn test_thought_type_case_insensitive() {
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("notes").unwrap(),
            ThoughtType::Notes
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("PROJECT").unwrap(),
            ThoughtType::Project
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("MiSc").unwrap(),
            ThoughtType::Misc
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("tOdO").unwrap(),
            ThoughtType::Todo
        ));
        assert!(matches!(
            <ThoughtType as FromStr>::from_str("question").unwrap(),
            ThoughtType::Question
        ));
    }

    #[test]
    fn test_thought_type_invalid_string() {
        assert!(<ThoughtType as FromStr>::from_str("InvalidType").is_err());
        assert!(<ThoughtType as FromStr>::from_str("").is_err());
        assert!(<ThoughtType as FromStr>::from_str("note").is_err());
    }

    #[test]
    fn test_thought_type_roundtrip_sql() {
        // Test that we can convert to string and back
        let original = ThoughtType::Project;
        let as_string = original.to_string();

        // Simulate SQL storage by converting through string
        let from_string = <ThoughtType as FromStr>::from_str(&as_string).unwrap();
        assert!(matches!(from_string, ThoughtType::Project));
    }

    #[test]
    fn test_thought_type_from_sql() {
        let value = ValueRef::Text(b"Project");
        let thought_type = ThoughtType::column_result(value).unwrap();
        assert!(matches!(thought_type, ThoughtType::Project));
    }

    #[test]
    fn test_thought_type_from_sql_case_insensitive() {
        let value = ValueRef::Text(b"todo");
        let thought_type = ThoughtType::column_result(value).unwrap();
        assert!(matches!(thought_type, ThoughtType::Todo));
    }

    #[test]
    fn test_args_getters() {
        let args = Args::new_for_test(ThoughtType::Notes, "Test content".to_string());
        assert!(matches!(args.thought_type(), ThoughtType::Notes));
        assert_eq!(args.content(), "Test content");
    }
}
