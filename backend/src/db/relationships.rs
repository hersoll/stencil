use anyhow::{Context, Result};
use sqlx::PgPool;

/// Generic trait for handling methods that update relationships (linking tables).
pub(crate) trait Relationship {
    const TABLE_NAME: &'static str;
    const PARENT_COLUMN: &'static str;
    const CHILD_COLUMN: &'static str;
}

pub(crate) struct CourseChapters;
impl Relationship for CourseChapters {
    const TABLE_NAME: &'static str = "course_chapters";
    const PARENT_COLUMN: &'static str = "course_id";
    const CHILD_COLUMN: &'static str = "chapter_id";
}

pub(crate) struct ChapterTopics;
impl Relationship for ChapterTopics {
    const TABLE_NAME: &'static str = "chapter_topics";
    const PARENT_COLUMN: &'static str = "chapter_id";
    const CHILD_COLUMN: &'static str = "topic_id";
}

pub(crate) struct TopicProblems;
impl Relationship for TopicProblems {
    const TABLE_NAME: &'static str = "topic_problems";
    const PARENT_COLUMN: &'static str = "topic_id";
    const CHILD_COLUMN: &'static str = "problem_id";
}

/// Generic helper for updating many-to-many relationships with ordering
pub(crate) async fn update_relationships<R: Relationship>(
    pool: &PgPool,
    parent_id: i32,
    child_ids: &[i32],
) -> Result<()> {
    // Clear existing relationships
    let delete_query = format!(
        "DELETE FROM {} WHERE {} = $1",
        R::TABLE_NAME,
        R::PARENT_COLUMN
    );
    sqlx::query(&delete_query)
        .bind(parent_id)
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to clear relationships in {} for {} = {}",
                R::TABLE_NAME,
                R::PARENT_COLUMN,
                parent_id
            )
        })?;

    // Start transaction for bulk insert
    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    // Insert new relationships with ordering
    let insert_query = format!(
        "INSERT INTO {} ({}, {}, order_index) VALUES ($1, $2, $3)",
        R::TABLE_NAME,
        R::PARENT_COLUMN,
        R::CHILD_COLUMN
    );

    for (order, child_id) in child_ids.iter().enumerate() {
        sqlx::query(&insert_query)
            .bind(parent_id)
            .bind(child_id)
            .bind((order + 1) as i32)
            .execute(&mut *tx)
            .await
            .with_context(|| {
                format!(
                    "Failed to add relationship: {} {} -> {} {}",
                    R::PARENT_COLUMN,
                    parent_id,
                    R::CHILD_COLUMN,
                    child_id
                )
            })?;
    }

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}
