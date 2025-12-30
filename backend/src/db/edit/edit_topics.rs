use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    db::{self, TopicEntry},
    errors::ApiError,
};

pub async fn get_topics() -> Result<impl IntoResponse, ApiError> {
    match db::topics::get_all_topic_data().await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_topic(Json(payload): Json<TopicEntry>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match db::topics::create_topic_from_entry(payload).await {
        Ok(id) => Ok((
            StatusCode::CREATED,
            format!("Created a new topic with an ID of {id}"),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn update_topic(Json(payload): Json<TopicEntry>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match db::topics::update_topic_from_entry(payload).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully updated {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Accepts an entire TopicEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_topic(Json(payload): Json<TopicEntry>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let id = payload.id;
    match db::topics::delete_topic_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all topics associated with a certain problem ID
pub async fn get_topics_from_problem(
    Path(problem_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem_id:#?}");
    match db::topics::get_topics_from_problem(&problem_id).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
