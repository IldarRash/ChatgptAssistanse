//! src/errors.rs

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

/// A custom error type for our application.
pub enum AppError {
    /// An error occurred with the Reqwest library.
    Reqwest(reqwest::Error),
    /// An error occurred during JSON serialization or deserialization.
    Serde(serde_json::Error),
    /// A generic error with a message.
    Anyhow(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Reqwest(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Request error: {}", err),
            ),
            AppError::Serde(err) => (
                StatusCode::BAD_REQUEST,
                format!("Serialization error: {}", err),
            ),
            AppError::Anyhow(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal error: {}", err),
            ),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}

/// Allows converting `reqwest::Error` into `AppError`.
impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Reqwest(err)
    }
}

/// Allows converting `serde_json::Error` into `AppError`.
impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serde(err)
    }
}

/// Allows converting `anyhow::Error` into `AppError`.
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Anyhow(err)
    }
}
