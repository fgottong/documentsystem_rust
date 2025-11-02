use crate::apierror::ApiError;
use crate::readinglist::ReadingList;
use axum::{Json, response::IntoResponse};
use serde_json::{Value, json};

/// handler-function//API-Endpoint just to check if server is running
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
            "status":"ok",
            "message":"Server läuft"
    }))
}

/// Test-Function, that always returns an error.
pub async fn get_err() -> Result<Json<Value>, ApiError> {
    Err(ApiError::InternalError)
}

/// Returns the current reading list...
pub async fn get_readinglist() -> Json<Value> {
    let reading_list = ReadingList::create_reading_list(); // TODO: Optimization: create reading_list once and reuse it;
    Json(reading_list.to_json())
}
