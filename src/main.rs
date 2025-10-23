pub mod document;
use crate::document::Document;

use crate::readinglist::ReadingList;
pub mod readinglist;

use axum::{
    Json, Router,
    http::{
        StatusCode, Uri,
        header::{self, HeaderMap, HeaderName},
    },
    response::{Html, IntoResponse},
    routing::get,
};
use serde_json::{Value, json};

#[derive(Debug)]
enum ApiError {
    NotFound,
    InvalidInput(String),
    InternalError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Data not Found".to_string()),
            ApiError::InvalidInput(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error".to_string(),
            ),
        };

        let body = Json(json!({"error":error_message}));

        (status, body).into_response()
    }
}

// handler-function//API-Endpoint just to check if server is running
async fn health_check() -> impl IntoResponse {
    Json(json!({
            "status":"ok",
            "message":"Server läuft"
    }))
}

async fn get_err() -> Result<Json<Value>, ApiError> {
    Err(ApiError::InternalError)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(health_check))
        .route("/err", get(get_err))
        .route("/readinglist", get(get_readinglist));

    let addr = "localhost:7878".to_string();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    // for book in &reading_list.list {
    //     let output = &book.to_string();
    //     println!("{}", output);
    //     println!();
    // }
    // let server = Server::new(addr.clone(), reading_list);
    // println!("Starte Server on : {}", addr);
    // server.run();
}

async fn get_readinglist() -> Json<Value> {
    //<String> {
    let reading_list = ReadingList::create_reading_list(); // TODO: Optimization: create reading_list once and reuse it;
    //Ok(Json(reading_list.to_json()))
    Json(reading_list.to_json())
}
