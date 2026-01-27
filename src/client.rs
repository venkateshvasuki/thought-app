use crate::{
    errors::AppError,
    reader_config::{AIClientConfig, AIClientDetails},
};
use lettre::transport::smtp::response;
use reqwest::blocking::{Client, Request};

pub fn get_request(
    client: &Client,
    config: &AIClientConfig,
    prompt: &String,
) -> Result<Request, AppError> {
    let request = client
        .post(config.ai_client().endpoint())
        .header("x-goog-api-key", config.bearer_token())
        .json(&prompt)
        .build()?;
    Ok(request)
}

pub fn send_request(client: &Client, request: Request) -> Result<String, AppError> {
    let response = client.execute(request)?;
    Ok(response.text()?)
}

pub fn get_response(config: &AIClientConfig, content: &[&String]) -> Result<String, AppError> {
    let prompt = format!(
        r#"I have a list of ideas - they could be startup ideas, product concepts, hobby projects, or experimental tools. For EACH idea below, provide relevant analysis and context.

**IMPORTANT**: 
- Assess the scale and nature of each idea (hobby project vs business venture vs research experiment)
- Only include sections that are meaningful for that specific idea
- If it's a hobby project, focus on learning opportunities, tech stack, and cool factor
- If it's a business idea, focus on market and monetization
- If unsure about something, say so
- Skip sections that don't apply

Consider including (when relevant):
- **Idea Type & Scale**: Is this a hobby project, side hustle, startup, or enterprise product?
- **What Makes This Interesting**: Why is this worth building or exploring?
- **Technical Approach**: Key technologies, architecture decisions, implementation complexity
- **Learning Opportunities**: New skills or concepts you'd gain (especially for hobby projects)
- **Market Analysis**: Market size, competitors, target users (for business ideas)
- **Business Model**: Revenue streams, monetization strategy (if applicable)
- **Time & Resource Estimate**: Rough effort required (hours/days/weeks/months)
- **Risks & Challenges**: Technical hurdles, market risks, or other obstacles
- **Similar Projects/Inspiration**: Existing tools or projects to learn from
- **Research Resources**: Relevant articles, papers, docs, or tutorials (with URLs when possible)
- **Next Steps**: Concrete first actions to validate or build this

**IDEAS:**

{:?}

For each idea:
- Use clear headings like "=== IDEA #1: [brief description] ==="
- Be practical and honest about feasibility
- Tailor your analysis to the idea's nature and scale
- Include any other insights particularly important for that idea"#,
        content
    );
    let escaped_prompt = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    // Construct request body as a string
    let request_body = format!(
        r#"{{"contents":[{{"parts":[{{"text":"{}"}}]}}]}}"#,
        escaped_prompt
    );
    let client = Client::new();

    let response = client
        .post(config.ai_client().endpoint())
        .header("x-goog-api-key", config.bearer_token())
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()?;

    // Debug: print status and raw response
    let status = response.status();
    let response_text = response.text()?;

    println!("Status: {}", status);
    println!("Raw response: {}", response_text);

    Ok(response_text)
    /*
        let client = Client::new();
        let request = get_request(&client, config, &request_body)?;
        let response = send_request(&client, request)?;
        Ok(response)
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_config(_endpoint: &str, token: &str) -> AIClientConfig {
        serde_json::from_str(&format!(
            r#"{{
                "bearer_token": "{}",
                "ai_client": "Gemini"
            }}"#,
            token
        ))
        .unwrap()
    }

    #[test]
    fn test_get_request_builds_correct_headers() {
        let config = create_test_config("http://test.com", "test_token");
        let client = Client::new();
        let prompt = "Test prompt".to_string();

        let request = get_request(&client, &config, &prompt).unwrap();

        assert_eq!(request.method(), "POST");
        assert!(request
            .headers()
            .get("x-goog-api-key")
            .unwrap()
            .to_str()
            .unwrap()
            .contains("test_token"));
    }

    #[test]
    fn test_get_request_builds_to_correct_endpoint() {
        let config = create_test_config("http://test.com", "api_key");
        let client = Client::new();
        let prompt = "Test prompt".to_string();

        let request = get_request(&client, &config, &prompt).unwrap();

        // Verify it posts to the AI client endpoint
        assert!(request.url().to_string().contains("generativelanguage.googleapis.com"));
    }

    #[test]
    fn test_get_request_includes_json_body() {
        let config = create_test_config("http://test.com", "api_key");
        let client = Client::new();
        let prompt = r#"{"test": "data"}"#.to_string();

        let request = get_request(&client, &config, &prompt).unwrap();

        assert!(request.body().is_some());
    }

    #[test]
    fn test_config_bearer_token_passed_to_header() {
        let expected_token = "my_secret_token_123";
        let config = create_test_config("http://test.com", expected_token);
        let client = Client::new();
        let prompt = "Test".to_string();

        let request = get_request(&client, &config, &prompt).unwrap();

        let header_value = request
            .headers()
            .get("x-goog-api-key")
            .unwrap()
            .to_str()
            .unwrap();

        assert_eq!(header_value, expected_token);
    }

    #[test]
    fn test_ai_client_config_structure() {
        let config = create_test_config("http://example.com", "token123");

        assert_eq!(config.bearer_token(), "token123");
        assert!(matches!(config.ai_client(), crate::reader_config::AIClient::Gemini));
    }
}
