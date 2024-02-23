use std::error::Error;

use diesel_async::{pooled_connection::{deadpool::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};

pub type AsyncPgConnectionPool = Pool<AsyncPgConnection>;

pub async fn create_database_pool(connection_string: &str) -> Result<AsyncPgConnectionPool, Box<dyn Error>> {
    // Configure connection
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_string);
    // Create pool
    let pool = Pool::builder(config)
        .max_size(5)
        .build()?;
    // Check if connection string is ok
    let _ = pool.get().await?;
    // Return pool
    Ok(pool)
}