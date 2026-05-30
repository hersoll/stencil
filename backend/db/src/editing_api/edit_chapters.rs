use crate::{
    ChapterEntry, chapters, courses,
    relationships::{self, ChapterTopics, CourseChapters},
    topics,
};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use types::errors::ApiError;

/// Returns all the data about every chapter in the DB as a `Vec<ChapterEntry>`.
///
/// This includes which courses/topics are linked to each chapter.
/// Used when listing every chapter in the editing list - since every entry will need data attached
/// to it when dragging into the editing area
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

/// Given a set of chapter IDs, returns data about those chapters as a `Vec<ChapterEntry>`
///
/// Used in the course/topic editing area, where that entry already knows which chapter IDs it's
/// connected to and simply wants to list those chapters in the list
pub async fn get_chapters_from_ids(
    Json(chapter_ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, ApiError> {
    match chapters::get_chapters_from_ids(&chapter_ids).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a chapter, including its connections to courses and topics, in the DB
pub async fn create_chapter(
    Json(chapter): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Received: {chapter:#?}",);
    let chapter_id = chapters::create_chapter_from_entry(&chapter).await?;
    relationships::update_children_for_parent::<ChapterTopics>(&chapter_id, &chapter.topic_ids)
        .await?;
    relationships::update_parents_for_child::<CourseChapters>(&chapter_id, &chapter.course_ids)
        .await?;

    Ok((
        StatusCode::OK,
        format!("Successfully created chapter with an ID of {chapter_id}"),
    ))
}

/// Update a chapter, including its connections to courses and topics, in the DB
pub async fn update_chapter(
    Json(chapter): Json<ChapterEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {chapter:#?}");
    let chapter_id = chapters::update_chapter_from_entry(&chapter).await?;
    relationships::update_children_for_parent::<ChapterTopics>(&chapter_id, &chapter.topic_ids)
        .await?;
    relationships::update_parents_for_child::<CourseChapters>(&chapter_id, &chapter.course_ids)
        .await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {}", chapter.name),
    ))
}

/// Delete a chapter, including its connections to courses and topics, from the DB
///
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
