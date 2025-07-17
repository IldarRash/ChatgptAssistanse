//! src/services/openai_service.rs

use crate::models::{
    Message, OpenAiRequest, OpenAiResponse, StructuredResponse,
};
use reqwest::Client;
use std::env;
use tracing::info;

/// Interacts with the OpenAI API to get structured data from a natural language query.
///
/// # Arguments
///
/// * `client` - A `reqwest::Client` to make HTTP requests.
/// * `input` - The user's natural language message.
///
/// # Returns
///
/// * A `Result` which is either the `StructuredResponse` from the AI
///   or a `reqwest::Error`.
pub async fn get_structured_response(
    client: &Client,
    input: &str,
) -> Result<StructuredResponse, anyhow::Error> {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    let system_prompt = "You are a helpful assistant that converts natural language into a structured JSON command. The user will provide a text, and you will return a JSON object with 'action', 'entity', and 'parameters'. For example, if the user says 'find all users named John', you should return: {\"action\": \"find\", \"entity\": \"users\", \"parameters\": [\"John\"]}. Only return the JSON object.";

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: system_prompt,
        },
        Message {
            role: "user".to_string(),
            content: input,
        },
    ];

    let openai_request = OpenAiRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages,
    };

    info!("Sending request to OpenAI");

    let openai_response = client
        .post("https://api.openai.com/v1/chat/completions")
        .bearer_auth(api_key)
        .json(&openai_request)
        .send()
        .await?
        .json::<OpenAiResponse>()
        .await?;

    info!("Received response from OpenAI");

    let content = &openai_response.choices[0].message.content;
    let structured_data: StructuredResponse = serde_json::from_str(content)?;

    Ok(structured_data)
}
