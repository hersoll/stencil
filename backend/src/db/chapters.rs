use super::common::{DbDescRow, error_context, error_context_by_name};
use crate::db::{self, ChapterEntry};
use anyhow::{Context, Result};

pub async fn get_all_chapter_data() -> Result<Vec<ChapterEntry>> {
    let pool = db::get_pool();
    let chapter_data = sqlx::query_as!(
        DbDescRow,
        r#"SELECT id, name, desc_sv, desc_en
            FROM chapters ORDER BY name"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(chapter_data.into_iter().map(ChapterEntry::from).collect())
}

pub async fn get_course_chapters(course_id: &i32) -> Result<Vec<ChapterEntry>> {
    let pool = db::get_pool();
    let chapters = sqlx::query_as!(
        DbDescRow,
        r#"SELECT ch.id, ch.name, ch.desc_sv, ch.desc_en
        FROM chapters ch
        JOIN course_chapters cc ON ch.id = cc.chapter_id
        WHERE cc.course_id = $1
        ORDER BY cc.order_index, ch.name"#,
        course_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters for course {}", course_id))?;

    Ok(chapters.into_iter().map(ChapterEntry::from).collect())
}

pub async fn get_chapters_from_topic(topic_id: &i32) -> Result<Vec<ChapterEntry>> {
    let pool = db::get_pool();
    let chapters = sqlx::query_as!(
        DbDescRow,
        r#"SELECT c.id, c.name, c.desc_sv, c.desc_en
        FROM chapters c
        JOIN chapter_topics ct ON c.id = ct.chapter_id
        WHERE ct.topic_id = $1
        ORDER BY c.name"#,
        topic_id
    )
    .fetch_all(pool)
    .await
    .with_context(|| format!("Failed to get chapters from topic {}", topic_id))?;

    Ok(chapters.into_iter().map(ChapterEntry::from).collect())
}

pub async fn create_chapter_from_entry(chapter: ChapterEntry) -> Result<i32> {
    let pool = db::get_pool();
    let created = sqlx::query!(
        r#"INSERT INTO chapters (name, desc_sv, desc_en) VALUES ($1, $2, $3) 
               RETURNING id"#,
        chapter.name,
        chapter.desc.sv,
        chapter.desc.en,
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "chapter", &chapter.name))?;

    Ok(created.id)
}

pub async fn update_chapter_from_entry(chapter: ChapterEntry) -> Result<String> {
    let pool = db::get_pool();
    let updated = sqlx::query!(
        r#"UPDATE chapters SET name = $1, desc_sv = $2, desc_en = $3 WHERE id = $4 
               RETURNING name"#,
        chapter.name,
        chapter.desc.sv,
        chapter.desc.en,
        chapter.id
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "chapter", chapter.id))?;

    Ok(updated.name)
}

pub async fn delete_chapter_with_id(id: i32) -> Result<String> {
    let pool = db::get_pool();
    let result = sqlx::query!(r#"DELETE FROM chapters WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "chapter", id))?;

    Ok(result.name)
}
