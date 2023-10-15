use axum::Router;

pub mod common;
pub mod static_files;

pub fn add_features(router: Router) -> Router {
    // Add all features
    let router = common::add_feature(router);
    let router = static_files::add_feature(router);
    // Return the new router
    router
}