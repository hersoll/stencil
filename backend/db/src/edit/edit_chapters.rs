use crate::{
    ChapterEntry, chapters, courses,
    relationships::{self, ChapterTopics, CourseChapters},
    topics,
};
use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;
use types::errors::ApiError;

pub async fn get_chapters() -> Result<impl IntoResponse, ApiError> {
    match chapters::get_all_chapter_data().await {
        Ok(mut chapters) => {
            for chapter in chapters.iter_mut() {
                let topics = topics::get_chapter_topics(&chapter.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                chapter.topic_ids = topics.iter().map(|t| t.id).collect();
                let courses = courses::get_courses_from_chapter(&chapter.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                chapter.course_ids = courses.iter().map(|c| c.id).collect();
            }
            Ok((StatusCode::OK, Json(json!(chapters))))
        }
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn get_chapters_from_ids(
    Json(chapter_ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, ApiError> {
    match chapters::get_chapters_from_ids(&chapter_ids).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_chapter(
    Json(chapter): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Received: {chapter:#?}",);
    let chapter_id = chapters::create_chapter_from_entry(&chapter).await?;
    relationships::update_children_for_parent::<ChapterTopics>(&chapter_id, &chapter.topic_ids)
        .await?;
    relationships::update_parents_for_child::<CourseChapters>(&chapter.course_ids, &chapter_id)
        .await?;

    Ok((
        StatusCode::OK,
        format!("Successfully created chapter with an ID of {chapter_id}"),
    ))
}

pub async fn update_chapter(
    Json(chapter): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {chapter:#?}",);
    let chapter_id = chapters::update_chapter_from_entry(&chapter).await?;
    relationships::update_children_for_parent::<ChapterTopics>(&chapter_id, &chapter.topic_ids)
        .await?;
    relationships::update_parents_for_child::<CourseChapters>(&chapter.course_ids, &chapter_id)
        .await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {}", chapter.name),
    ))
}

/// Accepts an entire ChapterEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_chapter(
    Json(payload): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let id = payload.id;
    match chapters::delete_chapter_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all chapters associated with a certain course ID
pub async fn get_chapters_from_course(
    Path(course_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {course_id:#?}");
    match chapters::get_course_chapters(&course_id).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all chapters associated with a certain topic ID
pub async fn get_chapters_from_topic(
    Path(topic_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic_id:#?}");
    match chapters::get_chapters_from_topic(&topic_id).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
