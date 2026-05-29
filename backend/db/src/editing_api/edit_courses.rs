use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    CourseEntry, chapters, courses,
    relationships::{self, CourseChapters},
};
use types::errors::ApiError;

pub async fn get_courses() -> Result<impl IntoResponse, ApiError> {
    match courses::get_all_course_data().await {
        Ok(mut courses) => {
            for course in courses.iter_mut() {
                let chapters = chapters::get_course_chapters(&course.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                course.chapter_ids = chapters.iter().map(|c| c.id).collect();
            }
            Ok((StatusCode::OK, Json(json!(courses))))
        }
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn get_courses_from_ids(
    Json(course_ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, ApiError> {
    match courses::get_courses_from_ids(&course_ids).await {
        Ok(courses) => Ok((StatusCode::OK, Json(json!(courses)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_course(
    Json(payload): Json<(CourseEntry, Vec<i32>)>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let (course, chapter_ids) = payload;
    let course_id = courses::create_course_from_entry(course).await?;
    relationships::update_children_for_parent::<CourseChapters>(&course_id, &chapter_ids).await?;

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
    relationships::update_children_for_parent::<CourseChapters>(&course.id, &chapter_ids).await?;
    let course_name = courses::update_course_from_entry(course).await?;

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
    match courses::delete_course_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all courses associated with a certain chapters ID
pub async fn get_courses_from_chapter(
    Path(chapter_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {chapter_id:#?}");
    match courses::get_courses_from_chapter(&chapter_id).await {
        Ok(courses) => Ok((StatusCode::OK, Json(json!(courses)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
