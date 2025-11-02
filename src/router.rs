use crate::endpoints::{self, Endpoints, RequestHandler};
use axum::{Router, routing::get};
use tokio::net::tcp::ReuniteError;

pub fn preperare_router() -> Router {
    let request_handler: Endpoints = Endpoints {};
    return create_router(request_handler);
}

pub fn create_router(rh: impl RequestHandler) -> Router {
    return Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| rh.health_check()))
        .route("/err", get(|| rh.get_err()))
        .route("/readinglist", get(|| rh.get_readinglist()));
}
