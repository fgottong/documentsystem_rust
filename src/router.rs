use crate::endpoints::*;
use axum::{Router, routing::get};
pub fn create_router() -> Router {
    return Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
        .route("/err", get(get_err))
        .route("/readinglist", get(get_readinglist));
}
