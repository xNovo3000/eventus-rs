use std::env;

use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenvy::dotenv;
use tracing::{info, debug, Level};
use tracing_appender::non_blocking::WorkerGuard;

pub mod dto;
pub mod model;
pub mod schema;

pub struct EnvironmentVariables {
    pub database_url: String
}

pub fn init_logger() -> WorkerGuard {
    // Get level based on build configuration
    let logger_level = if cfg!(debug_assertions) {
        Level::DEBUG
    } else {
        Level::ERROR
    };
    // Generate writer and drop guard
    let (writer, worker_guard) = if cfg!(debug_assertions) {
        // Debug build, log on terminal
        tracing_appender::non_blocking(std::io::stdout())
    } else {
        // Release build, log on rolling file
        let file_appender = tracing_appender::rolling::daily("logs", "eventus.log");
        tracing_appender::non_blocking(file_appender)
    };
    // Generate subscriber
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_max_level(logger_level)
        .init();
    // Return the guard (must be dropped at the end of the main function)
    worker_guard
}

pub fn init_environment_variables() -> Result<EnvironmentVariables, String> {
    // Load .env file if exists
    match dotenv() {
        Ok(_) => debug!("Loaded .env file"),
        Err(_) => info!(".env file not found")
    }
    // Try to load EnvironmentVariables struct
    Ok(EnvironmentVariables {
        database_url: env::var("DATABASE_URL")
            .map_err(|error| error.to_string())?
    })
}

pub async fn init_database_connection(database_url: &str) -> Result<impl AsyncConnection, String> {
    AsyncPgConnection::establish(database_url).await
        .map_err(|error| error.to_string())
}