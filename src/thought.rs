use crate::config::ThoughtType;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

#[derive(Debug)]
pub struct Thought {
    id: i32,
    thought_type: ThoughtType,
    content: String,
    reviewed: bool,
}

impl Thought {
    pub fn new(id: i32, thought_type: ThoughtType, content: String, reviewed: bool) -> Thought {
        Thought {
            id,
            thought_type,
            content,
            reviewed,
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
}
