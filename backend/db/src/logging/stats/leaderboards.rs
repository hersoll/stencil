use anyhow::Result;

use crate::{
    get_pool,
    logging::stats::{AggregationDuration, ValueCount},
};

/// Returns the number of **PDFs** the topic is included in, not the number of sets!
pub async fn most_used_topics(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = get_pool();
    let tops = sqlx::query_as!(
        ValueCount,
        r#"
            SELECT t.desc_sv as "value!", COUNT(DISTINCT pdf_id) AS "count!"
            FROM (
                SELECT pdf_id, UNNEST(topics) AS topic_id
                FROM logs_problem_set
            ) sub
            JOIN topics t ON t.id = sub.topic_id
            JOIN logs_pdf ON logs_pdf.id = sub.pdf_id
            WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
            GROUP BY t.id, t.desc_sv
            ORDER BY "count!" DESC 
            LIMIT 10;
        "#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(tops)
}

/// Returns the number of **sets** the problem is excluded from, not the number of PDFs!
/// This is due to the fact that a problem might be excluded from one set and included in the next,
/// and that is interesting data.
pub async fn most_excluded_problems(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = get_pool();
    let tops = sqlx::query_as!(
        ValueCount,
        r#"
            SELECT p.desc_sv as "value!", COUNT(*) AS "count!"
            FROM (
                SELECT pdf_id, UNNEST(exclusions) AS problem_id
                FROM logs_problem_set
            ) sub
            JOIN problems p ON p.id = sub.problem_id
            JOIN logs_pdf ON logs_pdf.id = sub.pdf_id
            WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
            GROUP BY p.id, p.desc_sv
            ORDER BY "count!" DESC 
            LIMIT 10;
        "#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(tops)
}
