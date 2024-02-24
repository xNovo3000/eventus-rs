use std::error::Error;

use tracing_appender::non_blocking::WorkerGuard;

use self::{config::AppConfiguration, database::AsyncPgConnectionPool};

pub mod config;
pub mod database;
pub mod logger;

pub struct AppState {
    pub config: AppConfiguration,
    pub postgres_pool: AsyncPgConnectionPool
}

pub async fn init() -> Result<(WorkerGuard, AppState), Box<dyn Error>> {
    // Initialize
    let config = config::load()?;
    let worker_guard = logger::init(&config.logger)?;
    let postgres_pool = database::create_database_pool(&config.datasource.postgres).await?;
    // Create state
    let state = AppState {
        config,
        postgres_pool
    };
    // Success
    Ok((worker_guard, state))
}