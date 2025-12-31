use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    db::{self, CourseEntry, relationships::CourseChapters},
    errors::ApiError,
};

pub async fn get_courses() -> Result<impl IntoResponse, ApiError> {
    match db::courses::get_all_course_data().await {
        Ok(courses) => Ok((StatusCode::OK, Json(json!(courses)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_course(
    Json(payload): Json<(CourseEntry, Vec<i32>)>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let (course, chapter_ids) = payload;
    let course_id = db::courses::create_course_from_entry(course).await?;
    db::relationships::update_children_for_parent::<CourseChapters>(&course_id, &chapter_ids)
        .await?;

    Ok((
        StatusCode::CREATED,
        format!("Created a new course with an ID of {course_id}"),
    ))
}

pub async fn update_course(
    Json(payload): Json<(CourseEntry, Vec<i32>)>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let (course, chapter_ids) = payload;
    db::relationships::update_children_for_parent::<CourseChapters>(&course.id, &chapter_ids)
        .await?;
    let course_name = db::courses::update_course_from_entry(course).await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {course_name}"),
    ))
}

/// Accepts an entire CourseEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_course(
    Json(payload): Json<CourseEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let id = payload.id;
    match db::courses::delete_course_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all courses associated with a certain chapters ID
pub async fn get_courses_from_chapter(
    Path(chapter_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {chapter_id:#?}");
    match db::courses::get_courses_from_chapter(&chapter_id).await {
        Ok(courses) => Ok((StatusCode::OK, Json(json!(courses)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
