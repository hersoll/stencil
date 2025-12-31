use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    db::{self, ProblemEntry, relationships::TopicProblems},
    errors::ApiError,
};

pub async fn get_problems() -> Result<impl IntoResponse, ApiError> {
    match db::problems::get_all_problem_data().await {
        Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_problem(
    Json(payload): Json<(ProblemEntry, Vec<i32>)>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let (problem, topic_ids) = payload;
    let problem_id = db::problems::create_problem_from_entry(&problem)
        .await
        .or_else(|e| Err(ApiError::Database(e.to_string())))?;
    match db::relationships::update_parents_for_child::<TopicProblems>(&topic_ids, &problem_id)
        .await
    {
        Ok(_) => Ok((
            StatusCode::OK,
            format!(
                "Successfully created {} with an id of {}",
                problem.name, problem_id
            ),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn update_problem(
    Json(payload): Json<(ProblemEntry, Vec<i32>)>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let (problem, topic_ids) = payload;
    db::relationships::update_parents_for_child::<TopicProblems>(&topic_ids, &problem.id)
        .await
        .or_else(|e| Err(ApiError::Database(e.to_string())))?;
    match db::problems::update_problem_from_entry(problem).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully updated {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Accepts an entire ProblemEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_problem(
    Json(payload): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let id = payload.id;
    match db::problems::delete_problem_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all problems associated with a certain topic ID
pub async fn get_problems_from_topic(
    Path(topic_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic_id:#?}");
    match db::problems::get_topic_problems(&topic_id).await {
        Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
