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

pub struct ThoughtsEmailBody<'a> {
    thoughts: &'a [Thought],
}

impl<'a> ThoughtsEmailBody<'a> {
    pub fn new(thoughts: &'a [Thought]) -> ThoughtsEmailBody<'a> {
        ThoughtsEmailBody { thoughts }
    }
}

impl IntoBody for ThoughtsEmailBody<'_> {
    fn into_body(self, _encoding: Option<ContentTransferEncoding>) -> Body {
        let body_text = if self.thoughts.is_empty() {
            r#"<html><body style="font-size: 16px;">
    <h2>Weekly Thoughts Summary</h2>
    <p>No thoughts recorded this week.</p>
    </body></html>"#
                .to_string()
        } else {
            let thoughts_section = self
                .thoughts
                .iter()
                .enumerate()
                .map(|(i, thought)| {
                    format!(
                        r#"<div style="font-size: 14px; margin-bottom: 20px;">
                <strong>{}. {}: </strong>
                <p>{}</p>
                <hr/>
                </div>"#,
                        i + 1,
                        thought.thought_type,
                        thought.content
                    )
                })
                .collect::<String>();

            format!(
                r#"<html><body style="font-size: 16px;">
        <h2>Weekly Thoughts Summary</h2>
        {}
        <p>End of weekly roundup</p>
        </body></html>"#,
                thoughts_section
            )
        };

        Body::new(body_text)
    }
}
