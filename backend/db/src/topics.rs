use crate::{
    DatabaseRow, DescriptionTranslations, HasDesc, ID, IsNew, NEW_THRESHOLD, Name, PublicFlag,
    error_context, error_context_by_name,
    problems::{ProblemIdsAndDifficulties, TopicSpecificData},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of data about a topic from the DB, the way it's sent to the user
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicEntry {
    pub id: ID,
    pub name: Name,
    pub desc: DescriptionTranslations,
    pub chapter_ids: Vec<ID>,
    pub problems: Vec<ProblemIdsAndDifficulties>,
    pub is_new: IsNew,
}
/// The same data as [`TopicEntry`], except it includes information about whether the entry is
/// public or not.
///
/// This is needed so the editor can edit the `public` value
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicEntryForEditor {
    #[serde(flatten)]
    pub entry: TopicEntry,
    pub public: PublicFlag,
}
impl HasDesc for TopicEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DatabaseRow> for TopicEntry {
    fn from(row: DatabaseRow) -> Self {
        TopicEntry {
            id: row.id,
            desc: row.as_desc_translations(),
            chapter_ids: Vec::new(),
            problems: Vec::new(),
            name: row.name,
            is_new: row.is_new,
        }
    }
}
impl From<DatabaseRow> for TopicEntryForEditor {
    fn from(row: DatabaseRow) -> Self {
        TopicEntryForEditor {
            public: row.public,
            entry: TopicEntry::from(row),
        }
    }
}

/// Retrieves data about *every* topic in the DB.
///
/// Used by the editor to list every topic
pub async fn get_all_topic_data() -> Result<Vec<TopicEntryForEditor>> {
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT id, name, desc_sv, desc_en, public,
            (created_at >= NOW() - $1::interval AND created_at >= DATE '2026-08-17') AS "is_new!"
            FROM topics ORDER BY name"#,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await?;

    Ok(topics.into_iter().map(TopicEntryForEditor::from).collect())
}

/// Retrieves data about every topic in the ID list.
///
/// Used by the user when editing a problem set: we want the description of the topic
/// TODO: We could theoretically pass it in the frontend somehow, if we want. That skips this entire
/// query.
pub async fn get_topics_from_ids(topic_ids: &[ID]) -> Result<Vec<TopicEntry>> {
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en, t.public,
            (created_at >= NOW() - $2::interval AND created_at >= DATE '2026-08-17') AS "is_new!"
        FROM topics t
        JOIN UNNEST($1::int[]) WITH ORDINALITY AS u(id, ord) ON t.id = u.id
        ORDER BY u.ord"#,
        topic_ids,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get topics with ids {topic_ids:?}"))?;

    Ok(topics.into_iter().map(TopicEntry::from).collect())
}

/// Ordered by chapter order_index
///
/// Editor version of [`get_topics_for_chapters()`]. Used when editing a chapter.
pub async fn get_topics_from_chapter(chapter_id: &i32) -> Result<Vec<TopicEntryForEditor>> {
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en, t.public,
            (created_at >= NOW() - $2::interval AND created_at >= DATE '2026-08-17') AS "is_new!"
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = $1
        ORDER BY ct.order_index, t.name"#,
        chapter_id,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get topics for chapter {}", chapter_id))?;

    Ok(topics.into_iter().map(TopicEntryForEditor::from).collect())
}

/// Gets every topic related to a problem
///
/// Used when editing problems
pub async fn get_topics_from_problem(problem_id: &i32) -> Result<Vec<TopicEntryForEditor>> {
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en, t.public,
            (created_at >= NOW() - $2::interval AND created_at >= DATE '2026-08-17') AS "is_new!"
        FROM topics t
        JOIN topic_problems tp ON t.id = tp.topic_id
        WHERE tp.problem_id = $1"#,
        problem_id,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get topics for problem {}", problem_id))?;

    Ok(topics.into_iter().map(TopicEntryForEditor::from).collect())
}

/// struct needed for [`get_topics_for_chapters()`]
struct SpecialTopicRow {
    id: i32,
    name: String,
    desc_sv: String,
    desc_en: String,
    chapter_id: i32,
    is_new: IsNew,
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
            is_new: value.is_new,
        }
    }
}
/// If we have multiple chapters (say, from a course)
/// we want to get all topics at the same time,
/// instead of hitting the DB for each chapter
///
/// User-facing version of [`get_topics_from_chapter()`]. Used in AddSetView when listing an entire
/// course's contents
pub async fn get_topics_for_chapters(chapter_ids: &[i32]) -> Result<HashMap<i32, Vec<TopicEntry>>> {
    let production_mode = crate::production_mode();
    let pool = crate::get_pool();
    let topics = sqlx::query_as!(
        SpecialTopicRow,
        r#"SELECT t.id, t.name, t.desc_sv, t.desc_en, ct.chapter_id,
            (created_at >= NOW() - $3::interval AND created_at >= DATE '2026-08-17') AS "is_new!"
        FROM topics t
        JOIN chapter_topics ct ON t.id = ct.topic_id
        WHERE ct.chapter_id = ANY($1)
        AND (NOT $2::bool OR t.public)
        ORDER BY ct.chapter_id, ct.order_index, t.name"#,
        chapter_ids,
        production_mode,
        NEW_THRESHOLD
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

/// Used by the editor to be able to list the difficulty for each problem in each topic
pub async fn get_topic_data_for_problem(problem_id: &i32) -> Result<Vec<TopicSpecificData>> {
    let pool = crate::get_pool();
    let topic_data = sqlx::query_as!(
        TopicSpecificData,
        r#"SELECT topic_id, absolute_difficulty, relative_difficulty
        FROM topic_problems
        WHERE problem_id = $1
        ORDER BY problem_id, topic_id"#,
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

/// Fetches topic data for multiple problems at once.
///
/// This is an optimization for the problem list in editor, instead of calling
/// [`get_topic_data_for_problem`] on every problem
pub async fn get_topic_data_for_problems(
    problem_ids: &[ID],
) -> Result<HashMap<ID, Vec<TopicSpecificData>>> {
    let pool = crate::get_pool();
    let topic_data = sqlx::query_as!(
        ProblemIdsAndDifficulties,
        r#"SELECT problem_id, topic_id, absolute_difficulty, relative_difficulty
        FROM topic_problems
        WHERE problem_id = ANY($1)
        ORDER BY problem_id, topic_id"#,
        problem_ids
    )
    .fetch_all(pool)
    .await
    .with_context(|| "Failed to get topic data for problems")?;

    let mut map: HashMap<ID, Vec<TopicSpecificData>> = HashMap::new();
    for row in topic_data {
        map.entry(row.problem_id)
            .or_default()
            .push(TopicSpecificData {
                topic_id: row.topic_id,
                absolute_difficulty: row.absolute_difficulty,
                relative_difficulty: row.relative_difficulty,
            });
    }
    Ok(map)
}

pub async fn create_topic_from_entry(topic: &TopicEntryForEditor) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO topics (name, desc_sv, desc_en, public) VALUES ($1, $2, $3, $4) 
               RETURNING id"#,
        topic.entry.name,
        topic.entry.desc.sv,
        topic.entry.desc.en,
        topic.public
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "topic", &topic.entry.name))?;

    Ok(created.id)
}

pub async fn update_topic_from_entry(topic: TopicEntryForEditor) -> Result<String> {
    let pool = crate::get_pool();
    let desc = topic.entry.desc;
    let updated = sqlx::query!(
        r#"UPDATE topics SET name = $1, desc_sv = $2, desc_en = $3, public = $4 WHERE id = $5 
               RETURNING name"#,
        topic.entry.name,
        desc.sv,
        desc.en,
        topic.public,
        topic.entry.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "topic", topic.entry.id))?;

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

/// Makes every topic public
pub async fn publish_all_topics() -> Result<u64> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE topics 
        SET public = true
        WHERE public = false"#,
    )
    .execute(pool)
    .await
    .with_context(|| "Failed to publish topics")?;

    Ok(updated.rows_affected())
}
