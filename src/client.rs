use crate::{
    errors::AppError,
    reader_config::{AIClientConfig, AIClientDetails},
};
use lettre::transport::smtp::response;
use reqwest::blocking::{Client, Request};

fn get_request(
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

fn send_request(client: &Client, request: Request) -> Result<String, AppError> {
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
    let client = Client::new();
    let request = get_request(&client, config, &prompt)?;
    let response = send_request(&client, request)?;
    Ok(response)
}
