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
                let topics = topics::get_topics_from_problem(&problem.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                let topic_ids: Vec<i32> = topics.iter().map(|t| t.id).collect();
                problem.topic_ids = topic_ids;
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
    relationships::update_parents_for_child::<TopicProblems>(&problem.topic_ids, &problem_id)
        .await?;
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
    relationships::update_parents_for_child::<TopicProblems>(&problem.topic_ids, &problem.id)
        .await?;
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

// TODO: Probably not in use, remove if nothing has broken
//
// /// Find and get all problems associated with a certain topic ID
// pub async fn get_problems_from_topic(
//     Path(topic_id): Path<i32>,
// ) -> Result<impl IntoResponse, ApiError> {
//     tracing::debug!("Recieved: {topic_id:#?}");
//     match problems::get_topic_problems(&topic_id).await {
//         Ok(problems) => Ok((StatusCode::OK, Json(json!(problems)))),
//         Err(e) => Err(ApiError::Database(e.to_string())),
//     }
// }
