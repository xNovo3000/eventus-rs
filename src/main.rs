use axum::{Router, Server};

pub mod core;
pub mod feature;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), String> {

    // Generate 
    let _logger_guard = core::init_logger();
    let environment_variables = core::init_environment_variables()?;
    let _database_connection = core::init_database_connection(&environment_variables.database_url).await?;

    // Create Axum web server and add features
    let router = Router::new();
    let router = feature::common::add_feature(router);

    // Bind to the port
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .map_err(|error| error.to_string())?;

    // Return success
    Ok(())
    
}