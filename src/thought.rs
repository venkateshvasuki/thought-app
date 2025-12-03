use crate::config::ThoughtType;

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

    pub fn thought_type(&self) -> &ThoughtType {
        &self.thought_type
    }
    pub fn content(&self) -> &String {
        &self.content
    }
    pub fn reviewed(&self) -> bool {
        self.reviewed
    }
}
