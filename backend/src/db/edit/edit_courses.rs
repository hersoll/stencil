use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    db::{self, CourseEntry},
    errors::ApiError,
};

pub async fn get_courses() -> Result<impl IntoResponse, ApiError> {
    match db::courses::get_all_course_data().await {
        Ok(courses) => Ok((StatusCode::OK, Json(json!(courses)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_course(
    Json(payload): Json<CourseEntry>,
) -> Result<impl IntoResponse, ApiError> {
    match db::courses::create_course_from_entry(payload).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            format!("Created a new course with an ID of {id}"),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn update_course(
    Json(payload): Json<CourseEntry>,
) -> Result<impl IntoResponse, ApiError> {
    match db::courses::update_course_from_entry(payload).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully updated {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Accepts an entire CourseEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_course(
    Json(payload): Json<CourseEntry>,
) -> Result<impl IntoResponse, ApiError> {
    let id = payload.id;
    match db::courses::delete_course_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
