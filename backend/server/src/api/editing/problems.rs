use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use db::{
    ID,
    problems::{self, ProblemEntryForEditor},
    relationships::{self, TopicProblems},
    topics,
};
use types::errors::ApiError;

/// Returns all the data about every problem in the DB as a `Vec<ProblemEntry>`.
///
/// This includes which topics are linked to each problem, and the difficulty of the problem in that
/// topic.
/// Used when listing every problem in the editing list - since every entry will need data attached
/// to it when dragging into the editing area
pub async fn get_problems() -> Result<impl IntoResponse, ApiError> {
    let mut problems = problems::get_all_problem_data().await?;
    let ids: Vec<ID> = problems.iter().map(|p| p.entry.id).collect();
    let mut topic_data = topics::get_topic_data_for_problems(&ids).await?;
    for problem in problems.iter_mut() {
        problem.entry.topic_data = topic_data.remove(&problem.entry.id).unwrap_or_default();
    }
    Ok((StatusCode::OK, Json(json!(problems))))
}

/// Given a topic ID, returns data about problems in that as a `Vec<ProblemEntry>`
///
/// Used in the topic editing area
pub async fn get_problems_from_topic(
    Path(topic_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    match problems::get_all_topic_problems_with_difficulties(&topic_id).await {
        Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a problem, including its connections to topics, in the DB
pub async fn create_problem(
    Json(problem): Json<ProblemEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    let problem_id = problems::create_problem_from_entry(&problem).await?;
    let topic_ids: Vec<i32> = problem
        .entry
        .topic_data
        .iter()
        .map(|topic| topic.topic_id)
        .collect();
    relationships::update_parents_for_child::<TopicProblems>(&problem_id, &topic_ids).await?;
    problems::update_difficulties_for_problem_with_id(&problem_id, &problem.entry.topic_data)
        .await?;

    Ok((
        StatusCode::OK,
        format!(
            "Successfully created {} with an id of {}",
            problem.entry.name, problem_id
        ),
    ))
}

/// Update a problem, including its connections to topics, in the DB
pub async fn update_problem(
    Json(problem): Json<ProblemEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    let topic_ids: Vec<i32> = problem
        .entry
        .topic_data
        .iter()
        .map(|topic| topic.topic_id)
        .collect();
    relationships::update_parents_for_child::<TopicProblems>(&problem.entry.id, &topic_ids).await?;
    problems::update_difficulties_for_problem_with_id(&problem.entry.id, &problem.entry.topic_data)
        .await?;
    let problem_name = problems::update_problem_from_entry(problem).await?;

    Ok((
        StatusCode::OK,
        format!("Successfully updated {problem_name}"),
    ))
}

/// Delete a problem, including its connections to topics, from the DB
///
/// Accepts an entire ProblemEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_problem(
    Json(problem): Json<ProblemEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    match problems::delete_problem_with_id(problem.entry.id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn publish_problems() -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved publish problem request");
    match problems::publish_all_problems().await {
        Ok(published_count) => Ok((
            StatusCode::OK,
            format!("Published {published_count} problems"),
        )),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
