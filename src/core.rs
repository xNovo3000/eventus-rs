use std::error::Error;

use config::{ConfigError, FileFormat, File, Config};
use diesel::ConnectionError;
use diesel_async::{AsyncPgConnection, AsyncConnection};
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;

/* LOGGER */

pub fn init_logger() -> Result<WorkerGuard, Box<dyn Error + Send + Sync + 'static>> {
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
    // Generate subscriber and return the guard if ok
    tracing_subscriber::fmt()
        .with_writer(writer)
        .with_max_level(logger_level)
        .try_init()
        .map(|_| worker_guard)
}

/* CONFIG */

#[derive(Clone, serde::Deserialize)]
pub struct AppConfiguration {
    pub database: Database
}

#[derive(Clone, serde::Deserialize)]
pub struct Database {
    pub postgres: Postgres
}

#[derive(Clone, serde::Deserialize)]
pub struct Postgres {
    pub url: String
}

pub fn load_configuration_files() -> Result<AppConfiguration, ConfigError> {
    // Create config builder
    let mut config_builder = Config::builder()
        .add_source(File::new("config/application.yaml", FileFormat::Yaml));
    // Override production
    if !cfg!(debug_assertions) {
        config_builder = config_builder.add_source(File::new("config/application-prod.yaml", FileFormat::Yaml));
    }
    // Build
    match config_builder.build() {
        Ok(config) => config.try_deserialize::<AppConfiguration>(),
        Err(config_error) => Err(config_error),
    }
}

/* DATABASE CONNECTIONS */

pub async fn init_database_connection(database_url: &str) -> Result<impl AsyncConnection, ConnectionError> {
    // Connecto to the database
    AsyncPgConnection::establish(database_url).await
}