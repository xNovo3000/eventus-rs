use std::error::Error;

use tracing_appender::non_blocking::WorkerGuard;

use self::database::AsyncPgConnectionPool;

pub mod config;
pub mod database;
pub mod logger;

pub struct AppState {
    pub postgres_pool: AsyncPgConnectionPool
}

pub async fn init() -> Result<(WorkerGuard, AppState), Box<dyn Error>> {
    // Initialize
    let config = config::load()?;
    let worker_guard = logger::init(&config.logger)?;
    let pool = database::create_database_pool(&config.datasource.postgres).await?;
    // Create state
    let state = AppState {
        postgres_pool: pool
    };
    // Success
    Ok((worker_guard, state))
}