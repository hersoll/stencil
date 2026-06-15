use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{
    ProblemIdsAndDifficulties, TopicEntry, chapters, problems,
    relationships::{self, ChapterTopics, TopicProblems},
    topics,
};
use types::errors::ApiError;

/// Returns all the data about every topic in the DB as a `Vec<TopicEntry>`.
///
/// This includes which chapters/problems are linked to each topic.
/// Used when listing every topic in the editing list - since every entry will need data attached
/// to it when dragging into the editing area
pub async fn get_topics() -> Result<impl IntoResponse, ApiError> {
    match topics::get_all_topic_data().await {
        Ok(mut topics) => {
            for topic in topics.iter_mut() {
                let problems = problems::get_topic_problems(&topic.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                topic.problems = problems
                    .iter()
                    .map(|p| ProblemIdsAndDifficulties::from_entry_and_topic_id(p, topic.id))
                    .collect();
                let chapters = chapters::get_chapters_from_topic(&topic.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                topic.chapter_ids = chapters.iter().map(|c| c.id).collect();
            }
            Ok((StatusCode::OK, Json(json!(topics))))
        }
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Given a set of topic IDs, returns data about those topics as a `Vec<TopicEntry>`
///
/// Used in the chapter/problem editing area, where that entry already knows which topic IDs it's
/// connected to and simply wants to list those topics in the list
pub async fn get_topics_from_ids(
    Json(topic_ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, ApiError> {
    match topics::get_topics_from_ids(&topic_ids).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a topic, including its connections to chapters and problems, in the DB
pub async fn create_topic(Json(topic): Json<TopicEntry>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic:#?}",);
    let topic_id = topics::create_topic_from_entry(&topic).await?;
    let problem_ids: Vec<i32> = topic.problems.iter().map(|p| p.problem_id).collect();
    relationships::update_children_for_parent::<TopicProblems>(&topic_id, &problem_ids).await?;
    relationships::update_parents_for_child::<ChapterTopics>(&topic_id, &topic.chapter_ids).await?;
    problems::update_difficulties_for_problems(&topic.problems).await?;

    Ok((
        StatusCode::CREATED,
        format!("Created a new topic with an ID of {topic_id}"),
    ))
}

/// Update a topic, including its connections to chapters and problems, in the DB
pub async fn update_topic(Json(topic): Json<TopicEntry>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic:#?}");
    let problem_ids: Vec<i32> = topic.problems.iter().map(|p| p.problem_id).collect();
    relationships::update_children_for_parent::<TopicProblems>(&topic.id, &problem_ids).await?;
    relationships::update_parents_for_child::<ChapterTopics>(&topic.id, &topic.chapter_ids).await?;
    problems::update_difficulties_for_problems(&topic.problems).await?;
    let topic_name = topics::update_topic_from_entry(topic).await?;

    Ok((StatusCode::OK, format!("Successfully updated {topic_name}")))
}

/// Delete a topic, including its connections to chapters and problems, from the DB
///
/// Accepts an entire TopicEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_topic(Json(payload): Json<TopicEntry>) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    let id = payload.id;
    match topics::delete_topic_with_id(id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
