use std::env;

use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenvy::dotenv;
use tracing::info;

pub mod dto;
pub mod handler;
pub mod model;
pub mod schema;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    // Load log writing with tracing
    let (writer, _guard) = if cfg!(debug_assertions) {
        // Debug build, log on terminal
        tracing_appender::non_blocking(std::io::stdout())
    } else {
        // Release build, log on rolling file
        let file_appender = tracing_appender::rolling::daily("logs", "eventus.log");
        tracing_appender::non_blocking(file_appender)
    };
    tracing_subscriber::fmt()
        .with_writer(writer)
        .init();
    // Try to load .env file
    match dotenv() {
        Ok(_) => info!("Loaded .env file"),
        Err(_) => info!(".env file not found")
    }
    // Get database URL
    let database_url = env::var("DATABASE_URL")
        .expect("Cannot load DATABASE_URL");
    // Connect to the database
    let _connection = AsyncPgConnection::establish(&database_url)
        .await
        .expect("Failed to connect to the database");
}