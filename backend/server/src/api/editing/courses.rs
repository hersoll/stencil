use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use db::{
    ID, chapters,
    courses::{self, CourseEntryForEditor},
    relationships::{self, CourseChapters},
};
use types::errors::ApiError;

/// Returns all the data about every course in the DB as a `Vec<CourseEntry>`.
///
/// This includes which chapters are linked to which course.
/// Used when listing every course in the editing list - since every entry will need data attached
/// to it when dragging into the editing area
pub async fn get_courses() -> Result<impl IntoResponse, ApiError> {
    let mut courses = courses::get_all_courses().await?;
    let ids: Vec<ID> = courses.iter().map(|c| c.entry.id).collect();
    let mut chapter_ids = chapters::get_chapter_ids_for_courses(&ids).await?;
    for course in courses.iter_mut() {
        course.entry.chapter_ids = chapter_ids.remove(&course.entry.id).unwrap_or_default();
    }
    Ok((StatusCode::OK, Json(json!(courses))))
}

/// Given a set of course IDs, returns data about those courses as a `Vec<CourseEntry>`
///
/// Used in the chapter editing area, where the chapter already knows which course IDs it belongs to
/// and simply wants to list those courses in the list
pub async fn get_courses_from_chapter(
    Path(chapter_id): Path<ID>,
) -> Result<impl IntoResponse, ApiError> {
    match courses::get_courses_from_chapter(&chapter_id).await {
        Ok(courses) => Ok((StatusCode::OK, Json(json!(courses)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a course, including its connections to chapters, in the DB
pub async fn create_course(
    Json(course): Json<CourseEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {course:#?}");
    let course_id = courses::create_course_from_entry(&course).await?;
    relationships::update_children_for_parent::<CourseChapters>(
        &course_id,
        &course.entry.chapter_ids,
    )
    .await?;

    Ok((
        StatusCode::CREATED,
        format!("Created a new course with an ID of {course_id}"),
    ))
}

/// Update a course, including its connections to chapters, in the DB
pub async fn update_course(
    Json(course): Json<CourseEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {course:#?}");
    relationships::update_children_for_parent::<CourseChapters>(
        &course.entry.id,
        &course.entry.chapter_ids,
    )
    .await?;
    let course_name = courses::update_course_from_entry(course).await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {course_name}"),
    ))
}

/// Delete a course (and its connections to chapters) from the DB.
///
/// Accepts an entire CourseEntry to keep ergonomics the same as [`create_course()`] and [`update_course()`]
/// If optimization is needed, can be made to only need an ID
pub async fn delete_course(
    Json(course): Json<CourseEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {course:#?}");
    match courses::delete_course_with_id(course.entry.id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
