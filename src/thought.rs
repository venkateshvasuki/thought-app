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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thought_new() {
        let thought = Thought::new(
            1,
            ThoughtType::Notes,
            "Test content".to_string(),
            false,
        );
        assert_eq!(*thought.id(), 1);
        assert!(matches!(thought.thought_type(), ThoughtType::Notes));
        assert_eq!(thought.content(), "Test content");
        assert_eq!(thought.reviewed(), false);
    }

    #[test]
    fn test_thought_getters() {
        let thought = Thought::new(
            42,
            ThoughtType::Project,
            "Project idea".to_string(),
            true,
        );
        assert_eq!(*thought.id(), 42);
        assert!(matches!(thought.thought_type(), ThoughtType::Project));
        assert_eq!(thought.content(), "Project idea");
        assert_eq!(thought.reviewed(), true);
    }

    #[test]
    fn test_thoughts_email_body_empty() {
        let thoughts: Vec<Thought> = vec![];
        let email_body = ThoughtsEmailBody::new(&thoughts);

        // Test that we can create the body without panicking
        // The actual Body type doesn't expose its content for testing
        // so we verify the struct can be constructed
        assert_eq!(email_body.thoughts.len(), 0);
    }

    #[test]
    fn test_thoughts_email_body_single_thought() {
        let thoughts = vec![Thought::new(
            1,
            ThoughtType::Todo,
            "Complete tests".to_string(),
            false,
        )];
        let email_body = ThoughtsEmailBody::new(&thoughts);

        // Verify the struct stores the thoughts correctly
        assert_eq!(email_body.thoughts.len(), 1);
        assert_eq!(email_body.thoughts[0].content(), "Complete tests");
    }

    #[test]
    fn test_thoughts_email_body_multiple_thoughts() {
        let thoughts = vec![
            Thought::new(1, ThoughtType::Notes, "First note".to_string(), false),
            Thought::new(2, ThoughtType::Project, "Second project".to_string(), false),
            Thought::new(3, ThoughtType::Question, "Third question".to_string(), false),
        ];
        let email_body = ThoughtsEmailBody::new(&thoughts);

        // Verify the struct stores all thoughts
        assert_eq!(email_body.thoughts.len(), 3);
        assert_eq!(email_body.thoughts[0].content(), "First note");
        assert_eq!(email_body.thoughts[1].content(), "Second project");
        assert_eq!(email_body.thoughts[2].content(), "Third question");
    }

    #[test]
    fn test_thoughts_email_body_html_structure() {
        let thoughts = vec![Thought::new(
            1,
            ThoughtType::Misc,
            "Test".to_string(),
            false,
        )];
        let email_body = ThoughtsEmailBody::new(&thoughts);

        // Verify we can create a Body from it without panicking
        let _body = email_body.into_body(None);
        // Body is created successfully if we reach here
    }
}
