use std::error::Error;

mod core;
mod model;
mod schema;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let config = core::load_config()?;
    let mut _pool = core::create_database_connection_pool(&config.datasource.clone().unwrap().postgres).await?;

    println!("{:?}", config);

    Ok(())
}
