use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use db::prefixes::{self, PrefixEntry};
use types::errors::ApiError;

/// Returns all the data about every prefix in the DB as a `Vec<PrefixEntry>`.
pub async fn get_prefixes() -> Result<impl IntoResponse, ApiError> {
    match prefixes::get_all_prefix_data().await {
        Ok(prefixes) => Ok((StatusCode::OK, Json(json!(prefixes)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Get a specific prefix using its ID
///
/// Used in the problem editor do display information about the connected prefix
pub async fn get_prefix_from_id(Path(prefix_id): Path<i32>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {prefix_id:#?}");
    match prefixes::get_prefix_from_id(&prefix_id).await {
        Ok(prefix) => Ok((StatusCode::OK, Json(json!(prefix)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a topic in the DB
pub async fn create_prefix(
    Json(payload): Json<PrefixEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match prefixes::create_prefix_from_entry(payload).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            format!("Created a new prefix with an ID of {id}"),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Update a prefix in the DB
pub async fn update_prefix(
    Json(payload): Json<PrefixEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match prefixes::update_prefix_from_entry(payload).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully updated {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
/// Delete a prefix from the DB
///
/// Accepts an entire PrefixEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_prefix(
    Json(payload): Json<PrefixEntry>,
) -> Result<impl IntoResponse, ApiError> {
    let id = payload.id;
    match prefixes::delete_prefix_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
