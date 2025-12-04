use crate::writer_config::ThoughtType;
use lettre::message::header::ContentTransferEncoding;
use lettre::message::{Body, IntoBody};
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

pub struct ThoughtsEmailBody<'a>(&'a [Thought]);

impl<'a> ThoughtsEmailBody<'a> {
    pub fn new(email: &'a [Thought]) -> ThoughtsEmailBody {
        ThoughtsEmailBody(email)
    }
}

impl IntoBody for ThoughtsEmailBody<'_> {
    fn into_body(self, _encoding: Option<ContentTransferEncoding>) -> Body {
        let mut body_text = String::from("Weekly Thoughts Summary\n");
        body_text.push_str("=======================\n\n");

        if self.0.is_empty() {
            body_text.push_str("No thoughts recorded this week.\n");
        } else {
            for (i, thought) in self.0.iter().enumerate() {
                body_text.push_str(&format!(
                    "{}. {}\n\n{}\n\n---\n\n",
                    i + 1,
                    thought.thought_type,
                    thought.content
                ));
            }
        }
        body_text.push_str("End of weekly roundup");

        Body::new(body_text)
    }
}
