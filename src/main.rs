use askama::Template;
use axum::{Router, Server, response::Html, routing::get};
use chrono::Local;
use template::IndexTemplate;
use tower_http::{trace::TraceLayer, compression::CompressionLayer, services::{ServeDir, ServeFile}};

pub mod core;
pub mod dto;
pub mod model;
pub mod schema;
pub mod template;

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    // Initialize application
    let _logger_guard = core::init_logger();
    let environment_variables = core::init_environment_variables();
    let _database_connection = core::init_database_connection(&environment_variables.database_url).await;

    // Create router and add features
    let router = Router::new()
        // Basic layers
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        // Static files
        .nest_service("/js", ServeDir::new("static/js"))
        .nest_service("/css", ServeDir::new("static/css"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
        // Index page
        .route("/", get(get_index_test));

    // Create Axum web server
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .expect("Cannot bind to the port");
    
}

pub async fn get_index_test() -> Html<String> {
    let template = IndexTemplate {
        time: Local::now()
    };
    Html(template.render().unwrap())
}