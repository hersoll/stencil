use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    db::{self, ProblemEntry},
    errors::ApiError,
};

pub async fn get_problems() -> Result<impl IntoResponse, ApiError> {
    match db::problems::get_all_problem_data().await {
        Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_problem(
    Json(payload): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    match db::problems::create_problem_from_entry(payload).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            format!("Created a new problem with an ID of {id}"),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn update_problem(
    Json(payload): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    match db::problems::update_problem_from_entry(payload).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully updated {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Accepts an entire ProblemEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_problem(
    Json(payload): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    let id = payload.id;
    match db::problems::delete_problem_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
