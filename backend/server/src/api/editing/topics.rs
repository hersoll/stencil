use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;

use db::{
    ID, chapters, problems,
    relationships::{self, ChapterTopics, TopicProblems},
    topics::{self, TopicEntryForEditor},
};
use types::errors::ApiError;

/// Returns all the data about every topic in the DB as a `Vec<TopicEntry>`.
///
/// This includes which chapters/problems are linked to each topic.
/// Used when listing every topic in the editing list - since every entry will need data attached
/// to it when dragging into the editing area
pub async fn get_topics() -> Result<impl IntoResponse, ApiError> {
    let mut topics = topics::get_all_topic_data().await?;
    let ids: Vec<ID> = topics.iter().map(|t| t.entry.id).collect();
    let mut problem_data = problems::get_topic_problems_with_difficulties_for_topics(&ids).await?;
    let mut chapter_ids = chapters::get_chapter_ids_for_topics(&ids).await?;
    for topic in topics.iter_mut() {
        topic.entry.problems = problem_data.remove(&topic.entry.id).unwrap_or_default();
        topic.entry.chapter_ids = chapter_ids.remove(&topic.entry.id).unwrap_or_default();
    }
    Ok((StatusCode::OK, Json(json!(topics))))
}

/// Given a chapter ID, returns data about related topics as a `Vec<TopicEntry>`
///
/// Used in the chapter editing area
pub async fn get_topics_from_chapter(
    Path(chapter_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    match topics::get_topics_from_chapter(&chapter_id).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Given a problem ID, returns data about related topics as a `Vec<TopicEntry>`
///
/// Used in the problem editing area
pub async fn get_topics_from_problem(
    Path(problem_id): Path<i32>,
) -> Result<impl IntoResponse, ApiError> {
    match topics::get_topics_from_problem(&problem_id).await {
        Ok(topics) => Ok((StatusCode::OK, Json(json!(topics)))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}

/// Create a topic, including its connections to chapters and problems, in the DB
pub async fn create_topic(
    Json(topic): Json<TopicEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic:#?}",);
    let topic_id = topics::create_topic_from_entry(&topic).await?;
    let problem_ids: Vec<i32> = topic.entry.problems.iter().map(|p| p.problem_id).collect();
    relationships::update_children_for_parent::<TopicProblems>(&topic_id, &problem_ids).await?;
    relationships::update_parents_for_child::<ChapterTopics>(&topic_id, &topic.entry.chapter_ids)
        .await?;
    problems::update_difficulties_for_problems(&topic.entry.problems).await?;

    Ok((
        StatusCode::CREATED,
        format!("Created a new topic with an ID of {topic_id}"),
    ))
}

/// Update a topic, including its connections to chapters and problems, in the DB
pub async fn update_topic(
    Json(topic): Json<TopicEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {topic:#?}");
    let problem_ids: Vec<i32> = topic.entry.problems.iter().map(|p| p.problem_id).collect();
    relationships::update_children_for_parent::<TopicProblems>(&topic.entry.id, &problem_ids)
        .await?;
    relationships::update_parents_for_child::<ChapterTopics>(
        &topic.entry.id,
        &topic.entry.chapter_ids,
    )
    .await?;
    problems::update_difficulties_for_problems(&topic.entry.problems).await?;
    let topic_name = topics::update_topic_from_entry(topic).await?;

    Ok((StatusCode::OK, format!("Successfully updated {topic_name}")))
}

/// Delete a topic, including its connections to chapters and problems, from the DB
///
/// Accepts an entire TopicEntry to keep ergonomics the same
/// If optimization is needed, can be made to only need an ID
pub async fn delete_topic(
    Json(payload): Json<TopicEntryForEditor>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::debug!("Recieved: {payload:#?}");
    match topics::delete_topic_with_id(payload.entry.id).await {
        Ok(name) => Ok((StatusCode::OK, format!("Successfully deleted {name}"))),
        Err(e) => Err(ApiError::Database(e.to_string())),
    }
}
