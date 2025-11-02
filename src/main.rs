pub mod apierror;
pub mod category;
pub mod document;
pub mod documentsystem;
pub mod endpoints;
pub mod readinglist;
pub mod router;

use crate::router::create_router;

use crate::documentsystem::DocumentSystem;

#[tokio::main]
async fn main() {
    let doc_sys: DocumentSystem = DocumentSystem::new();

    let app = create_router();
    let addr = "localhost:7878".to_string();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
