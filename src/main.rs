use std::error::Error;

use axum::Router;
use tokio::net::TcpListener;

pub mod api;
pub mod core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initalize app
    let (_guard, state) = core::init().await?;
    // Build application
    let app = Router::new();
    // Create listener
    let listener = TcpListener::bind(format!("{}:{}", "0.0.0.0", &state.config.server.port)).await?;
    // Serve content
    axum::serve(listener, app).await?;
    // Success
    Ok(())
}