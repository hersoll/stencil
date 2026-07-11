use anyhow::Result;

use crate::logging::stats::{AggregationDuration, ValueCount};

// Most functions in this module are identical apart from the field they query.
// We could do this dynamically but then we lose type safety (what happens if I rename a DB column?).

pub async fn get_set_column_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT ps.columns as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY ps.columns;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_set_heading_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT ps.has_heading as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY ps.has_heading;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_set_spacing_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT ps.problem_spacing as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY ps.problem_spacing;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_set_page_break_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT ps.pagebreak_after as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY ps.pagebreak_after;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_set_starting_difficulty_count(
    duration: AggregationDuration,
) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT ps.starting_difficulty as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY ps.starting_difficulty;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_set_ending_difficulty_count(
    duration: AggregationDuration,
) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT ps.ending_difficulty as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY ps.ending_difficulty;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_set_topic_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT array_length(ps.topics, 1) as value, COUNT(*) as "count!" 
        FROM logs_problem_set as ps INNER JOIN logs_pdf ON ps.pdf_id = logs_pdf.id
        WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY array_length(ps.topics, 1);"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}
