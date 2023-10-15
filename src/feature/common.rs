use axum::Router;
use tower_http::{trace::TraceLayer, compression::CompressionLayer};

pub fn add_feature(router: Router) -> Router {
    router
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
}