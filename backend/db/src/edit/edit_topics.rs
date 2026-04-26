use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;

use crate::{
    TopicEntry, chapters, problems,
    relationships::{self, ChapterTopics, TopicProblems},
    topics,
};
use types::errors::ApiError;

#[derive(Debug, Deserialize)]
pub struct TopicPayload {
    topic: TopicEntry,
    chapters: Vec<i32>,
    problems: Vec<i32>,
}

pub async fn get_topics() -> Result<impl IntoResponse, ApiError> {
    match topics::get_all_topic_data().await {
        Ok(mut topics) => {
            for topic in topics.iter_mut() {
                let problems = problems::get_topic_problems(&topic.id)
                    .await
                    .map_err(|e| ApiError::Database(e.to_string()))?;
                topic.problem_ids = problems.iter().map(|p| p.id).collect();
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

pub async fn get_topics_from_ids(
    Json(topic_ids): Json<Vec<i32>>,
) -> Result<impl IntoResponse, ApiError> {
    match topics::get_topics_from_ids(&topic_ids).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

pub async fn create_topic(
    Json(payload): Json<TopicPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!(
        "Recieved: {:#?} {:#?} {:#?}",
        payload.topic,
        payload.chapters,
        payload.problems
    );
    let topic_id = topics::create_topic_from_entry(payload.topic).await?;
    relationships::update_children_for_parent::<TopicProblems>(&topic_id, &payload.problems)
        .await?;
    relationships::update_parents_for_child::<ChapterTopics>(&payload.chapters, &topic_id).await?;

    Ok((
        StatusCode::CREATED,
        format!("Created a new topic with an ID of {topic_id}"),
    ))
}

pub async fn update_topic(
    Json(payload): Json<TopicPayload>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!(
        "Recieved: {:#?} {:#?} {:#?}",
        payload.topic,
        payload.chapters,
        payload.problems
    );
    relationships::update_children_for_parent::<TopicProblems>(
        &payload.topic.id,
        &payload.problems,
    )
    .await?;
    relationships::update_parents_for_child::<ChapterTopics>(&payload.chapters, &payload.topic.id)
        .await?;
    let topic_name = topics::update_topic_from_entry(payload.topic).await?;

    Ok((StatusCode::OK, format!("Successfully updated {topic_name}")))
}

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

/// Find and get all topics associated with a certain problem ID
pub async fn get_topics_from_problem(
    Path(problem_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {problem_id:#?}");
    match topics::get_topics_from_problem(&problem_id).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Find and get all topics associated with a certain chapter ID
pub async fn get_topics_from_chapter(
    Path(chapter_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {chapter_id:#?}");
    match topics::get_chapter_topics(&chapter_id).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
