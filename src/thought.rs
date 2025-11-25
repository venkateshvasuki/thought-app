#[derive(Debug)]
pub struct Thought {
    pub id: i32,
    pub thought_type: String,
    pub content: String,
    pub reviewed: bool,
}
