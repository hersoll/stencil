use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{
    db::{
        self, ChapterEntry,
        relationships::{ChapterTopics, CourseChapters},
    },
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

#[derive(Debug, Deserialize)]
pub struct UpdateChapterPayload {
    chapter: ChapterEntry,
    courses: Vec<i32>,
    topics: Vec<i32>,
}

pub async fn update_chapter(
    Json(payload): Json<UpdateChapterPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!(
        "Recieved: {:#?} {:#?} {:#?}",
        payload.chapter,
        payload.courses,
        payload.topics
    );
    db::relationships::update_children_for_parent::<ChapterTopics>(
        &payload.chapter.id,
        &payload.topics,
    )
    .await
    .or_else(|e| Err(ApiError::Database(e.to_string())))?;
    db::relationships::update_parents_for_child::<CourseChapters>(
        &payload.courses,
        &payload.chapter.id,
    )
    .await
    .or_else(|e| Err(ApiError::Database(e.to_string())))?;
    match db::chapters::update_chapter_from_entry(payload.chapter).await {
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

/// Find and get all chapters associated with a certain course ID
pub async fn get_chapters_from_course(
    Path(course_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {course_id:#?}");
    match db::chapters::get_course_chapters(&course_id).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all chapters associated with a certain topic ID
pub async fn get_chapters_from_topic(
    Path(topic_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic_id:#?}");
    match db::chapters::get_chapters_from_topic(&topic_id).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
