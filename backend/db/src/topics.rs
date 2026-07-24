use std::collections::HashMap;

use super::{DbDescRow, error_context, error_context_by_name};
use crate::{
    DescriptionTranslations, HasDesc,
    problems::{ProblemIdsAndDifficulties, TopicSpecificData},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// Representation of data about a topic from the DB
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicEntry {
    pub id: i32,
    pub name: String,
    pub desc: DescriptionTranslations,
    pub chapter_ids: Vec<i32>,
    pub problems: Vec<ProblemIdsAndDifficulties>,
}
impl HasDesc for TopicEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DbDescRow> for TopicEntry {
    fn from(row: DbDescRow) -> Self {
        let (id, name, desc) = row.into_desc_translations();
        TopicEntry {
            id,
            name,
            desc,
            chapter_ids: Vec::new(),
            problems: Vec::new(),
        }
    }
}

pub async fn get_all_topic_data() -> Result<Vec<TopicEntry>> {
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en
            FROM topics ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(topics.into_iter().map(TopicEntry::from).collect())
}

pub async fn get_topics_from_ids(topic_ids: &[i32]) -> Result<Vec<TopicEntry>> {
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        DbDescRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en
        FROM topics t
        JOIN UNNEST($1::int[]) WITH ORDINALITY AS u(id, ord) ON t.id = u.id
        ORDER BY u.ord"#,
        topic_ids
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get topics with ids {topic_ids:?}"))?;

    Ok(topics.into_iter().map(TopicEntry::from).collect())
}

/// Ordered by chapter order_index
///
/// Editor version of [`get_topics_for_chapters()`].
pub async fn get_chapter_topics(chapter_id: &i32) -> Result<Vec<TopicEntry>> {
    let pool = crate::get_pool();
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
impl From<SpecialTopicRow> for TopicEntry {
    fn from(value: SpecialTopicRow) -> Self {
        TopicEntry {
            id: value.id,
            name: value.name,
            desc: DescriptionTranslations {
                sv: value.desc_sv,
                en: value.desc_en,
            },
            chapter_ids: Vec::new(),
            problems: Vec::new(),
        }
    }
}
/// If we have multiple chapters (say, from a course)
/// we want to get all topics at the same time,
/// instead of hitting the DB for each chapter
///
/// User-facing version of [`get_chapter_topics()`].
/// NOTE: Other functions with the production_mode bool also has a
/// `ForceReadPrivateData` parameter since it can be accessed by the editor. This function cannot.
pub async fn get_topics_for_chapters(chapter_ids: &[i32]) -> Result<HashMap<i32, Vec<TopicEntry>>> {
    // In prod we only want the public rows,
    // in dev we want all
    let production_mode = cfg!(feature = "docker") || std::env::args().any(|x| x == "prod");

    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        SpecialTopicRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en, ct.chapter_id
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = ANY($1)
        AND (NOT $2::bool OR t.public)
        ORDER BY ct.chapter_id, ct.order_index, t.name"#,
        chapter_ids,
        production_mode
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

pub async fn get_topic_data_for_problem(problem_id: &i32) -> Result<Vec<TopicSpecificData>> {
    let pool = crate::get_pool();
    let topic_data = sqlx::query_as!(
        TopicSpecificData,
        r#"SELECT topic_id, absolute_difficulty, relative_difficulty
        FROM topic_problems
        WHERE problem_id = $1
        ORDER BY topic_id"#,
        problem_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| {
        format!(
            "Failed to get topic data for problem with id {}",
            problem_id
        )
    })?;

    Ok(topic_data)
}

pub async fn create_topic_from_entry(topic: &TopicEntry) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO topics (name, desc_sv, desc_en) VALUES ($1, $2, $3) 
               RETURNING id"#,
        topic.name,
        topic.desc.sv,
        topic.desc.en,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "topic", &topic.name))?;

    Ok(created.id)
}

pub async fn update_topic_from_entry(topic: TopicEntry) -> Result<String> {
    let pool = crate::get_pool();
    let desc = topic.desc;
    let updated = sqlx::query!(
        r#"UPDATE topics SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 
               RETURNING name"#,
        topic.name,
        desc.sv,
        desc.en,
        topic.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "topic", topic.id))?;

    Ok(updated.name)
}

pub async fn delete_topic_with_id(id: i32) -> Result<String> {
    let pool = crate::get_pool();
    let _result = sqlx::query!(r#"DELETE FROM chapter_topics WHERE topic_id = $1"#, id)
        .execute(pool)
        .await
        .with_context(|| error_context("delete", "topic", id))?;
    let _result = sqlx::query!(r#"DELETE FROM topic_problems WHERE topic_id = $1"#, id)
        .execute(pool)
        .await
        .with_context(|| error_context("delete", "topic", id))?;

    let result = sqlx::query!(r#"DELETE FROM topics WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "topic", id))?;

    Ok(result.name)
}
