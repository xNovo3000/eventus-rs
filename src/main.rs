use std::error::Error;

mod auth;
mod core;
mod model;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = core::load_config()?;
    let _logger_guard = core::init_logger(&config.logger.clone().unwrap())?;
    let mut _pool = core::create_database_connection_pool(&config.datasource.clone().unwrap().postgres).await?;

    println!("{:?}", config);

    Ok(())
}
