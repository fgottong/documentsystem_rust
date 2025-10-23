pub mod apierror;
pub mod document;
pub mod endpoints;
pub mod readinglist;
pub mod router;

use crate::router::create_router;

#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = "localhost:7878".to_string();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
