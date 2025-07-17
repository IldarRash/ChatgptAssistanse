//! src/models.rs

use serde::{Deserialize, Serialize};

/// A request from the user containing the chat message.
#[derive(Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

/// The structured response we expect from the AI model.
#[derive(Serialize, Deserialize, Debug)]
pub struct StructuredResponse {
    pub action: String,
    pub entity: String,
    pub parameters: Vec<String>,
}

/// The request payload for the OpenAI API.
#[derive(Serialize, Debug)]
pub struct OpenAiRequest<'a> {
    pub model: String,
    pub messages: Vec<Message<'a>>,
}

/// A message in the chat history for the OpenAI API.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message<'a> {
    pub role: String,
    pub content: &'a str,
}

/// The response from the OpenAI API.
#[derive(Deserialize, Debug)]
pub struct OpenAiResponse {
    pub choices: Vec<Choice>,
}

/// A choice in the OpenAI API response.
#[derive(Deserialize, Debug)]
pub struct Choice {
    pub message: OpenAiMessage,
}

/// A message returned from the OpenAI API.
#[derive(Deserialize, Debug)]
pub struct OpenAiMessage {
    pub role: String,
    pub content: String,
}
