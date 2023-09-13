use std::env;

use diesel_async::{AsyncPgConnection, AsyncConnection};
use dotenvy::dotenv;
use tracing::error;

pub mod dto;
pub mod handler;
pub mod model;
pub mod schema;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), ()> {
    // Load tracing
    tracing_subscriber::fmt::init();
    // Try to load .env file
    if let Err(error) = dotenv() {
        error!("Cannot load .env file: {error}");
        return Err(())
    }
    // Get database URL
    let database_url = env::var("DATABASE_URL")
        .expect("Cannot load DATABASE_URL");
    // Connect to the database
    let _connection = AsyncPgConnection::establish(&database_url)
        .await
        .expect("Failed to connect to the database");
    // Ok
    Ok(())
}