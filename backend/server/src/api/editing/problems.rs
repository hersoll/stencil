use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use db::{
    ProblemEntry, problems,
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

/// Given a topic ID, returns data about problems in that as a `Vec<ProblemEntry>`
///
/// Note that this breaks the pattern of the other entries (which had something like `get_problems_from_ids`),
/// since we need the difficulties for problems and to do that we need to know which topic we're
/// talking about, we can't simply extract problems in a vacuum.
///
/// Used in the topic editing area, where that entry already knows which problem IDs it's
/// connected to and simply wants to list those problems in the list
pub async fn get_problems_from_topic_id(
    Json(topic_id): Json<i32>,
) -> Result<impl IntoResponse, ApiError> {
    match problems::get_topic_problems(&topic_id).await {
        Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a problem, including its connections to topics, in the DB
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
    relationships::update_parents_for_child::<TopicProblems>(&problem_id, &topic_ids).await?;
    problems::update_difficulties_for_problem_with_id(&problem_id, &problem.topic_data).await?;

    Ok((
        StatusCode::OK,
        format!(
            "Successfully created {} with an id of {}",
            problem.name, problem_id
        ),
    ))
}

/// Update a problem, including its connections to topics, in the DB
pub async fn update_problem(
    Json(problem): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    let topic_ids: Vec<i32> = problem
        .topic_data
        .iter()
        .map(|topic| topic.topic_id)
        .collect();
    relationships::update_parents_for_child::<TopicProblems>(&problem.id, &topic_ids).await?;
    problems::update_difficulties_for_problem_with_id(&problem.id, &problem.topic_data).await?;
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
    Json(problem): Json<ProblemEntry>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem:#?}");
    match problems::delete_problem_with_id(problem.id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
