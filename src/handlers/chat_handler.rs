//! src/handlers/chat_handler.rs

use crate::{
    errors::AppError,
    models::{ChatRequest, StructuredResponse},
    services::openai_service,
};
use axum::{extract::State, Json};
use reqwest::Client;

/// Handles incoming chat requests.
/// It takes a user's message, sends it to the OpenAI service,
/// and returns the structured response.
pub async fn chat_handler(
    State(client): State<Client>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<StructuredResponse>, AppError> {
    let response =
        openai_service::get_structured_response(&client, &payload.message).await?;
    Ok(Json(response))
}
