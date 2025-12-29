use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    db::{self, ChapterEntry},
    errors::ApiError,
};

pub async fn get_chapters() -> Result<impl IntoResponse, ApiError> {
    match db::chapters::get_all_chapter_data().await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_chapter(
    Json(payload): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match db::chapters::create_chapter_from_entry(payload).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            format!("Created a new chapter with an ID of {id}"),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn update_chapter(
    Json(payload): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match db::chapters::update_chapter_from_entry(payload).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully updated {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Accepts an entire ChapterEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_chapter(
    Json(payload): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let id = payload.id;
    match db::chapters::delete_chapter_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
