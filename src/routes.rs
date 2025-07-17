//! src/routes.rs

use crate::handlers::chat_handler::chat_handler;
use axum::{routing::post, Router};
use reqwest::Client;

/// Creates the main application router.
pub fn create_router() -> Router {
    let client = Client::new();

    Router::new()
        .route("/chat", post(chat_handler))
        .with_state(client)
}
