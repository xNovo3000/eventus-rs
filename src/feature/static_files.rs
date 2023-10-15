use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn add_feature(router: Router) -> Router {
    router
        .nest_service("/js", ServeDir::new("static/js"))
        .nest_service("/css", ServeDir::new("static/css"))
        .nest_service("/favicon.ico", ServeFile::new("static/favicon.ico"))
}