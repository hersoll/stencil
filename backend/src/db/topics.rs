use std::collections::HashMap;

use super::common::{DbDescRow, error_context, error_context_by_name};
use super::relationships::{TopicProblems, update_relationships};
use crate::db::{self, DescriptionTranslations, TopicEntry};
use anyhow::{Context, Result};

impl From<DbDescRow> for TopicEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        TopicEntry { id, name, desc }
    }
}

/// Get all topics ordered by name
pub async fn get_all_topic_data() -> Result<Vec<TopicEntry>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en
            FROM topics ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(topics.into_iter().map(TopicEntry::from).collect())
}

/// Get all topics for a specific chapter, ordered by chapter order_index
pub async fn get_chapter_topics(chapter_id: i32) -> Result<Vec<TopicEntry>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        DbDescRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = $1
        ORDER BY ct.order_index, t.name"#,
        chapter_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get topics for chapter {}", chapter_id))?;

    Ok(topics.into_iter().map(TopicEntry::from).collect())
}

/// struct needed for get_topics_for_chapters()
struct SpecialTopicRow {
    id: i32,
    name: String,
    desc_sv: String,
    desc_en: String,
    chapter_id: i32,
}
impl Into<TopicEntry> for SpecialTopicRow {
    fn into(self) -> TopicEntry {
        TopicEntry {
            id: self.id,
            name: self.name,
            desc: DescriptionTranslations {
                sv: self.desc_sv,
                en: self.desc_en,
            },
        }
    }
}
/// If we have multiple chapters (say, from a course) we want to get all topics at the same time,
/// instead of hitting the DB for each chapter
pub async fn get_topics_for_chapters(chapter_ids: &[i32]) -> Result<HashMap<i32, Vec<TopicEntry>>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        SpecialTopicRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en, ct.chapter_id
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = ANY($1)
        ORDER BY ct.chapter_id, ct.order_index, t.name"#,
        chapter_ids
    )
    .fetch_all(pool)
    .await?;

    // Group by chapter_id
    let mut map: HashMap<i32, Vec<TopicEntry>> = HashMap::new();
    for row in topics {
        map.entry(row.chapter_id).or_default().push(row.into());
    }
    Ok(map)
}

/// Get a single topic by ID
pub async fn get_topic(id: i32) -> Result<TopicEntry> {
    let pool = db::get_pool();
    let topic = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en
                FROM topics
                WHERE id = $1"#,
        id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("get", "topic", id))?;

    Ok(TopicEntry::from(topic))
}

/// Get multiple topics by IDs
pub async fn get_topics(ids: &[i32]) -> Result<Vec<TopicEntry>> {
    let pool = db::get_pool();
    let topics = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en 
                FROM topics
                WHERE id = ANY($1)
                ORDER BY name"#,
        ids,
    )
    .fetch_all(pool)
    .await?;

    Ok(topics.into_iter().map(TopicEntry::from).collect())
}

/// Create a new topic
pub async fn create_topic(topic: TopicEntry) -> Result<TopicEntry> {
    let pool = db::get_pool();
    let desc = topic.desc;
    let created = sqlx::query_as!(
        DbDescRow,
        r#"INSERT INTO topics (name, desc_sv, desc_en) VALUES ($1, $2, $3) 
               RETURNING id, name, desc_sv, desc_en"#,
        topic.name,
        desc.sv,
        desc.en,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "topic", &topic.name))?;

    Ok(TopicEntry::from(created))
}

/// Update an existing topic
pub async fn update_topic(topic: TopicEntry) -> Result<TopicEntry> {
    let pool = db::get_pool();
    let desc = topic.desc;
    let updated = sqlx::query_as!(
        DbDescRow,
        r#"UPDATE topics SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 
               RETURNING id, name, desc_sv, desc_en"#,
        topic.name,
        desc.sv,
        desc.en,
        topic.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "topic", topic.id))?;

    Ok(TopicEntry::from(updated))
}

/// Delete a topic by ID, returns the deleted topic name
pub async fn delete_topic(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM topics WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "topic", id))?;

    Ok(result.name)
}

/// Update the problems associated with a topic
pub async fn update_topic_problems(topic_id: i32, problem_ids: Vec<i32>) -> Result<()> {
    let pool = db::get_pool();
    update_relationships::<TopicProblems>(pool, topic_id, &problem_ids).await
}
