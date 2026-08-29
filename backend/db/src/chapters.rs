use crate::{
    DatabaseRow, DescriptionTranslations, HasDesc, ID, IsNew, NEW_THRESHOLD, Name, PublicFlag,
    error_context, error_context_by_name,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of data about a chapter from the DB, as it is sent to the user
#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterEntry {
    pub id: ID,
    pub name: Name,
    pub desc: DescriptionTranslations,
    pub course_ids: Vec<ID>,
    pub topic_ids: Vec<ID>,
    pub is_new: IsNew,
}
/// The same data as [`ChapterEntry`], except it includes information about whether the entry is
/// public or not.
///
/// This is needed so the editor can edit the `public` value
#[derive(Debug, Serialize, Deserialize)]
pub struct ChapterEntryForEditor {
    #[serde(flatten)]
    pub entry: ChapterEntry,
    pub public: PublicFlag,
}
impl HasDesc for ChapterEntry {
    fn desc(&self) -> &DescriptionTranslations {
        &self.desc
    }
}
impl From<DatabaseRow> for ChapterEntry {
    fn from(row: DatabaseRow) -> Self {
        ChapterEntry {
            id: row.id,
            desc: row.as_desc_translations(),
            course_ids: Vec::new(),
            topic_ids: Vec::new(),
            name: row.name,
            is_new: row.is_new,
        }
    }
}
impl From<DatabaseRow> for ChapterEntryForEditor {
    fn from(row: DatabaseRow) -> Self {
        ChapterEntryForEditor {
            public: row.public,
            entry: ChapterEntry::from(row),
        }
    }
}

/// Returns all the data about every chapter, public or private
///
/// Used by the editor to list all the chapters
pub async fn get_all_chapter_data() -> Result<Vec<ChapterEntryForEditor>> {
    let pool = crate::get_pool();
    let chapter_data = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT id, name, desc_sv, desc_en, public,
            (created_at >= NOW() - $1::interval) AS "is_new!"
            FROM chapters ORDER BY name"#,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await?;

    Ok(chapter_data
        .into_iter()
        .map(ChapterEntryForEditor::from)
        .collect())
}

/// Gets every chapter marked as `public` related to the course (unless in dev mode).
///
/// Used by the user when a course is selected and the chapters need to be listed in AddSetView
pub async fn get_public_chapters_from_course(course_id: &i32) -> Result<Vec<ChapterEntry>> {
    let production_mode = crate::production_mode();
    let pool = crate::get_pool();
    let chapters = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT ch.id, ch.name, ch.desc_sv, ch.desc_en, ch.public,
            (created_at >= NOW() - $3::interval) AS "is_new!"
        FROM chapters ch
        JOIN course_chapters cc ON ch.id = cc.chapter_id
        WHERE cc.course_id = $1
        AND (NOT $2::bool OR ch.public)
        ORDER BY cc.order_index, ch.name"#,
        course_id,
        production_mode,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters for course {}", course_id))?;

    Ok(chapters.into_iter().map(ChapterEntry::from).collect())
}

/// Gets every chapter related to the course
///
/// Used by the editor when editing a course and need the related chapters listed
pub async fn get_all_chapters_from_course(course_id: &i32) -> Result<Vec<ChapterEntryForEditor>> {
    let production_mode = crate::production_mode();
    let pool = crate::get_pool();
    let chapters = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT ch.id, ch.name, ch.desc_sv, ch.desc_en, ch.public,
            (created_at >= NOW() - $3::interval) AS "is_new!"
        FROM chapters ch
        JOIN course_chapters cc ON ch.id = cc.chapter_id
        WHERE cc.course_id = $1
        AND (NOT $2::bool OR ch.public)
        ORDER BY cc.order_index, ch.name"#,
        course_id,
        production_mode,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters for course {}", course_id))?;

    Ok(chapters
        .into_iter()
        .map(ChapterEntryForEditor::from)
        .collect())
}

/// Fetches every chapter related to each course, collected as a HashMap.
///
/// Used when loading the initial course list in the editor
pub async fn get_chapter_ids_for_courses(course_ids: &[ID]) -> Result<HashMap<ID, Vec<ID>>> {
    let pool = crate::get_pool();
    let rows = sqlx::query!(
        r#"SELECT ch.id, cc.course_id
        FROM chapters ch
        JOIN course_chapters cc ON ch.id = cc.chapter_id
        WHERE cc.course_id = ANY($1)
        ORDER BY cc.course_id, cc.order_index, ch.name"#,
        course_ids,
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters for courses {:?}", course_ids))?;

    let mut map: HashMap<ID, Vec<ID>> = HashMap::new();
    for row in rows {
        let course_id = row.course_id;
        map.entry(course_id).or_default().push(row.id);
    }
    Ok(map)
}

/// Gets every chapter related to the topic
///
/// Used by the editor when editing a topic and need the related chapters listed
pub async fn get_chapters_from_topic(topic_id: &i32) -> Result<Vec<ChapterEntryForEditor>> {
    let pool = crate::get_pool();
    let chapters = sqlx::query_as!(
        DatabaseRow,
        r#"SELECT c.id, c.name, c.desc_sv, c.desc_en, c.public,
            (created_at >= NOW() - $2::interval) AS "is_new!"
        FROM chapters c
        JOIN chapter_topics ct ON c.id = ct.chapter_id
        WHERE ct.topic_id = $1
        ORDER BY c.name"#,
        topic_id,
        NEW_THRESHOLD
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters from topic {}", topic_id))?;

    Ok(chapters
        .into_iter()
        .map(ChapterEntryForEditor::from)
        .collect())
}

/// Fetches every chapter related to each topic, collected as a HashMap.
///
/// Used when loading the initial topic list in the editor
pub async fn get_chapter_ids_for_topics(topic_ids: &[ID]) -> Result<HashMap<ID, Vec<ID>>> {
    let pool = crate::get_pool();
    let rows = sqlx::query!(
        r#"SELECT c.id, ct.topic_id
        FROM chapters c
        JOIN chapter_topics ct ON c.id = ct.chapter_id
        WHERE ct.topic_id = ANY($1)
        ORDER BY ct.topic_id, c.name"#,
        topic_ids
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters from topics {:?}", topic_ids))?;

    let mut map: HashMap<ID, Vec<ID>> = HashMap::new();
    for row in rows {
        let topic_id = row.topic_id;
        map.entry(topic_id).or_default().push(row.id);
    }
    Ok(map)
}

pub async fn create_chapter_from_entry(chapter: &ChapterEntryForEditor) -> Result<i32> {
    let pool = crate::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO chapters (name, desc_sv, desc_en, public) VALUES ($1, $2, $3, $4)
               RETURNING id"#,
        chapter.entry.name,
        chapter.entry.desc.sv,
        chapter.entry.desc.en,
        chapter.public
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "chapter", &chapter.entry.name))?;

    Ok(created.id)
}

pub async fn update_chapter_from_entry(chapter: &ChapterEntryForEditor) -> Result<i32> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE chapters SET name = $1, desc_sv = $2, desc_en = $3, public = $4 WHERE id = $5
               RETURNING id"#,
        chapter.entry.name,
        chapter.entry.desc.sv,
        chapter.entry.desc.en,
        chapter.public,
        chapter.entry.id,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "chapter", chapter.entry.id))?;

    Ok(updated.id)
}

pub async fn delete_chapter_with_id(id: i32) -> Result<String> {
    let pool = crate::get_pool();
    let _result = sqlx::query!(r#"DELETE FROM chapter_topics WHERE chapter_id = $1"#, id)
        .execute(pool)
        .await
        .with_context(|| error_context("delete", "chapter", id))?;
    let _result = sqlx::query!(r#"DELETE FROM course_chapters WHERE chapter_id = $1"#, id)
        .execute(pool)
        .await
        .with_context(|| error_context("delete", "chapter", id))?;
    let result = sqlx::query!(r#"DELETE FROM chapters WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "chapter", id))?;

    Ok(result.name)
}
/// Makes every chapter public
pub async fn publish_all_chapters() -> Result<u64> {
    let pool = crate::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE chapters 
        SET public = true
        WHERE public = false"#,
    )
    .execute(pool)
    .await
    .with_context(|| "Failed to publish chapters")?;

    Ok(updated.rows_affected())
}
