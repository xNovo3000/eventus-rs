use std::env;

use askama::Template;
use axum::{Router, routing::get, Server, response::Html};
use chrono::Local;
use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenvy::dotenv;
use template::IndexTemplate;
use tracing::{info, Level};

pub mod dto;
pub mod handler;
pub mod model;
pub mod schema;
pub mod template;

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    // Load logging system
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
        .with_max_level(if cfg!(debug_assertions) { Level::DEBUG } else { Level::ERROR })
        .init();

    // Try to load .env file
    match dotenv() {
        Ok(_) => info!("Loaded .env file"),
        Err(_) => info!(".env file not found")
    }
    // Get environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("Cannot load DATABASE_URL");

    // Connect to the database
    let _connection = AsyncPgConnection::establish(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Create Axum web server
    let app = Router::new()
        .route("/", get(get_index));

    // Bind to the port
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn get_index() -> Html<String> {
    IndexTemplate { time: Local::now() }
        .render()
        .unwrap()
        .into()
}