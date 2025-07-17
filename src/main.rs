//! src/main.rs

mod errors;
mod handlers;
mod models;
mod routes;
mod services;

use dotenv::dotenv;
use routes::create_router;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

/// The main entry point of the application.
#[tokio::main]
async fn main() {
    // Initialize the logger.
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // Load environment variables.
    dotenv().ok();

    // Create the application router.
    let app = create_router();

    // Define the server address.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());

    // Start the server.
    axum::serve(listener, app).await.unwrap();
}
