use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use db::{
    ID,
    chapters::{self, ChapterEntryForEditor},
    courses,
    relationships::{self, ChapterTopics, CourseChapters},
    topics,
};
use types::errors::ApiError;

/// Returns all the data about every chapter in the DB as a `Vec<ChapterEntry>`.
///
/// This includes which courses/topics are linked to each chapter.
/// Used when listing every chapter in the editing list - since every entry will need data attached
/// to it when dragging into the editing area
pub async fn get_chapters() -> Result<impl IntoResponse, ApiError> {
    let mut chapters = chapters::get_all_chapter_data().await?;
    let ids: Vec<ID> = chapters.iter().map(|c| c.entry.id).collect();
    let mut topics = topics::get_topics_for_chapters(&ids).await?;
    let mut courses = courses::get_courses_for_chapters(&ids).await?;
    for chapter in chapters.iter_mut() {
        chapter.entry.topic_ids = topics
            .remove(&chapter.entry.id)
            .unwrap_or_default()
            .iter()
            .map(|t| t.id)
            .collect();
        chapter.entry.course_ids = courses.remove(&chapter.entry.id).unwrap_or_default();
    }

    Ok((StatusCode::OK, Json(json!(chapters))))
}

/// Given a course ID, returns data about related chapters as a `Vec<ChapterEntry>`
///
/// Used in the course editing area
pub async fn get_chapters_from_course(
    Path(course_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    match chapters::get_all_chapters_from_course(&course_id).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Given a topic ID, returns data about related chapters as a `Vec<ChapterEntry>`
///
/// Used in the topic editing area
pub async fn get_chapters_from_topic(
    Path(topic_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    match chapters::get_chapters_from_topic(&topic_id).await {
        Ok(chapters) => Ok((StatusCode::OK, Json(json!(chapters)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a chapter, including its connections to courses and topics, in the DB
pub async fn create_chapter(
    Json(chapter): Json<ChapterEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Received: {chapter:#?}",);
    let chapter_id = chapters::create_chapter_from_entry(&chapter).await?;
    relationships::update_children_for_parent::<ChapterTopics>(
        &chapter_id,
        &chapter.entry.topic_ids,
    )
    .await?;
    relationships::update_parents_for_child::<CourseChapters>(
        &chapter_id,
        &chapter.entry.course_ids,
    )
    .await?;

    Ok((
        StatusCode::OK,
        format!("Successfully created chapter with an ID of {chapter_id}"),
    ))
}

/// Update a chapter, including its connections to courses and topics, in the DB
pub async fn update_chapter(
    Json(chapter): Json<ChapterEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {chapter:#?}");
    let chapter_id = chapters::update_chapter_from_entry(&chapter).await?;
    relationships::update_children_for_parent::<ChapterTopics>(
        &chapter_id,
        &chapter.entry.topic_ids,
    )
    .await?;
    relationships::update_parents_for_child::<CourseChapters>(
        &chapter_id,
        &chapter.entry.course_ids,
    )
    .await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {}", chapter.entry.name),
    ))
}

/// Delete a chapter, including its connections to courses and topics, from the DB
///
/// Accepts an entire ChapterEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_chapter(
    Json(payload): Json<ChapterEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match chapters::delete_chapter_with_id(payload.entry.id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
