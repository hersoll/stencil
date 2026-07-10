use anyhow::{Context, Result};

/// Generic trait for handling methods that update relationships (linking tables).
pub trait Relationship {
    const TABLE_NAME: &'static str;
    const PARENT_COLUMN: &'static str;
    const CHILD_COLUMN: &'static str;
}

pub struct CourseChapters;
impl Relationship for CourseChapters {
    const TABLE_NAME: &'static str = "course_chapters";
    const PARENT_COLUMN: &'static str = "course_id";
    const CHILD_COLUMN: &'static str = "chapter_id";
}

pub struct ChapterTopics;
impl Relationship for ChapterTopics {
    const TABLE_NAME: &'static str = "chapter_topics";
    const PARENT_COLUMN: &'static str = "chapter_id";
    const CHILD_COLUMN: &'static str = "topic_id";
}

pub struct TopicProblems;
impl Relationship for TopicProblems {
    const TABLE_NAME: &'static str = "topic_problems";
    const PARENT_COLUMN: &'static str = "topic_id";
    const CHILD_COLUMN: &'static str = "problem_id";
}

/// Generic helper for updating many-to-many relationships with ordering
pub async fn update_children_for_parent<R: Relationship>(
    parent_id: &i32,
    child_ids: &[i32],
) -> Result<()> {
    let pool = crate::get_pool();
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

/// Sync parents for a given child while preserving per-parent order_index.
/// - Removes parents not in `parent_ids`
/// - Keeps existing order_index values
/// - Appends the child to new parents at the end
pub async fn update_parents_for_child<R: Relationship>(
    child_id: &i32,
    parent_ids: &[i32],
) -> Result<()> {
    let pool = crate::get_pool();

    let mut tx = pool.begin().await.context("Failed to start transaction")?;

    // 1. Delete relationships for this child that are NOT in parent_ids
    let delete_query = format!(
        "DELETE FROM {table}
         WHERE {child_col} = $1
           AND {parent_col} <> ALL($2)",
        table = R::TABLE_NAME,
        parent_col = R::PARENT_COLUMN,
        child_col = R::CHILD_COLUMN,
    );

    sqlx::query(&delete_query)
        .bind(child_id)
        .bind(parent_ids)
        .execute(&mut *tx)
        .await
        .context("Failed to delete removed parent relationships")?;

    // 2. Insert missing (parent, child) relationships
    let insert_query = format!(
        r#"
        INSERT INTO {table} ({parent_col}, {child_col}, order_index)
        SELECT
            p.parent_id,
            $1,
            COALESCE(
                (
                    SELECT MAX(order_index) + 1
                    FROM {table} t
                    WHERE t.{parent_col} = p.parent_id
                ),
                1
            )
        FROM UNNEST($2::int[]) AS p(parent_id)
        WHERE NOT EXISTS (
            SELECT 1
            FROM {table} t
            WHERE t.{parent_col} = p.parent_id
              AND t.{child_col} = $1
        )
        "#,
        table = R::TABLE_NAME,
        parent_col = R::PARENT_COLUMN,
        child_col = R::CHILD_COLUMN,
    );

    sqlx::query(&insert_query)
        .bind(child_id)
        .bind(parent_ids)
        .execute(&mut *tx)
        .await
        .context("Failed to insert new parent relationships")?;

    tx.commit().await.context("Failed to commit transaction")?;

    Ok(())
}
