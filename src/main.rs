use axum::{Router, Server};

pub mod core;
pub mod feature;

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    // Initialize application
    let _logger_guard = core::init_logger();
    let environment_variables = core::init_environment_variables();
    let _database_connection = core::init_database_connection(&environment_variables.database_url).await;

    // Create Axum web server, add features and bind to the port
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(feature::add_features(Router::new()).into_make_service())
        .await
        .expect("Cannot bind to the port");
    
}