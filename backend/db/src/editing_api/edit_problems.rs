use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    ProblemEntry, problems,
    relationships::{self, TopicProblems},
    topics,
};
use types::errors::ApiError;

pub async fn get_problems() -> Result<impl IntoResponse, ApiError> {
    match problems::get_all_problem_data().await {
        Ok(mut problems) => {
            for problem in problems.iter_mut() {
                let topic_data = topics::get_topic_data_for_problem(&problem.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                problem.topic_data = topic_data;
            }
            Ok((StatusCode::OK, Json(json!(problems))))
        }
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn get_problems_from_ids(
    Json(problem_ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, ApiError> {
    match problems::get_problems_from_ids(&problem_ids).await {
        Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_problem(
    Json(problem): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    let problem_id = problems::create_problem_from_entry(&problem).await?;
    let topic_ids: Vec<i32> = problem
        .topic_data
        .iter()
        .map(|topic| topic.topic_id)
        .collect();
    relationships::update_parents_for_child::<TopicProblems>(&topic_ids, &problem_id).await?;
    problems::update_difficulties_for_problem_with_id(&problem_id, &problem.topic_data).await?;

    Ok((
        StatusCode::OK,
        format!(
            "Successfully created {} with an id of {}",
            problem.name, problem_id
        ),
    ))
}

pub async fn update_problem(
    Json(problem): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    let topic_ids: Vec<i32> = problem
        .topic_data
        .iter()
        .map(|topic| topic.topic_id)
        .collect();
    relationships::update_parents_for_child::<TopicProblems>(&topic_ids, &problem.id).await?;
    problems::update_difficulties_for_problem_with_id(&problem.id, &problem.topic_data).await?;
    let problem_name = problems::update_problem_from_entry(problem).await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {problem_name}"),
    ))
}

/// Accepts an entire ProblemEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_problem(
    Json(problem): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    match problems::delete_problem_with_id(problem.id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
