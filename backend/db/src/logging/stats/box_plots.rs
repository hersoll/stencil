use anyhow::Result;
use serde::Serialize;

use crate::{get_pool, logging::stats::AggregationDuration};

#[derive(Serialize)]
pub struct BoxPlotData<T> {
    min: T,
    // Postgres calculates percentiles as floats
    p10: f64,
    p25: f64,
    median: f64,
    mean: f64,
    p75: f64,
    p90: f64,
    max: T,
}

pub async fn render_times(duration: AggregationDuration) -> Result<BoxPlotData<f64>> {
    let pool = get_pool();
    let box_plot = sqlx::query_as!(
        BoxPlotData,
        r#"
            SELECT
                EXTRACT(EPOCH FROM MIN(time_taken))::float8 AS "min!",
                EXTRACT(EPOCH FROM PERCENTILE_CONT(0.10) WITHIN GROUP (ORDER BY time_taken))::float8 AS "p10!",
                EXTRACT(EPOCH FROM PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY time_taken))::float8 AS "p25!",
                EXTRACT(EPOCH FROM PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY time_taken))::float8 AS "median!",
                EXTRACT(EPOCH FROM PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY time_taken))::float8 AS "p75!",
                EXTRACT(EPOCH FROM PERCENTILE_CONT(0.90) WITHIN GROUP (ORDER BY time_taken))::float8 AS "p90!",
                EXTRACT(EPOCH FROM MAX(time_taken))::float8 AS "max!",
                EXTRACT(EPOCH FROM AVG(time_taken))::float8 AS "mean!"
            FROM logs_pdf
            WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        "#,
        duration.to_interval()
    )
    .fetch_one(pool)
    .await?;

    Ok(box_plot)
}

pub async fn topics_per_set(duration: AggregationDuration) -> Result<BoxPlotData<i32>> {
    let pool = get_pool();
    let box_plot = sqlx::query_as!(
        BoxPlotData,
        r#"
            SELECT
                MIN(array_length(topics, 1)) AS "min!",
                PERCENTILE_CONT(0.10) WITHIN GROUP (ORDER BY array_length(topics, 1)) AS "p10!",
                PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY array_length(topics, 1)) AS "p25!",
                PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY array_length(topics, 1)) AS "median!",
                PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY array_length(topics, 1)) AS "p75!",
                PERCENTILE_CONT(0.90) WITHIN GROUP (ORDER BY array_length(topics, 1)) AS "p90!",
                MAX(array_length(topics, 1)) AS "max!",
                AVG(array_length(topics, 1))::float8 AS "mean!"
            FROM logs_problem_set JOIN logs_pdf ON logs_problem_set.pdf_id = logs_pdf.id
            WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        "#,
        duration.to_interval()
    )
    .fetch_one(pool)
    .await?;

    Ok(box_plot)
}

pub async fn exclusions_per_set(duration: AggregationDuration) -> Result<BoxPlotData<i32>> {
    let pool = get_pool();
    let box_plot = sqlx::query_as!(
        BoxPlotData,
        // COALESCE since exclusions can be empty => NULL length in 1-dimension
        r#"
            SELECT
                MIN(COALESCE(array_length(exclusions, 1), 0)) AS "min!",
                PERCENTILE_CONT(0.10) WITHIN GROUP (ORDER BY COALESCE(array_length(exclusions, 1), 0)) AS "p10!",
                PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY COALESCE(array_length(exclusions, 1), 0)) AS "p25!",
                PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY COALESCE(array_length(exclusions, 1), 0)) AS "median!",
                PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY COALESCE(array_length(exclusions, 1), 0)) AS "p75!",
                PERCENTILE_CONT(0.90) WITHIN GROUP (ORDER BY COALESCE(array_length(exclusions, 1), 0)) AS "p90!",
                MAX(COALESCE(array_length(exclusions, 1), 0)) AS "max!",
                AVG(COALESCE(array_length(exclusions, 1), 0))::float8 AS "mean!"
            FROM logs_problem_set JOIN logs_pdf ON logs_problem_set.pdf_id = logs_pdf.id
            WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        "#,
        duration.to_interval()
    )
    .fetch_one(pool)
    .await?;

    Ok(box_plot)
}

pub async fn problem_count_per_set(duration: AggregationDuration) -> Result<BoxPlotData<i32>> {
    let pool = get_pool();
    let box_plot = sqlx::query_as!(
        BoxPlotData,
        r#"
            SELECT
                MIN(problem_count) AS "min!",
                PERCENTILE_CONT(0.10) WITHIN GROUP (ORDER BY problem_count) AS "p10!",
                PERCENTILE_CONT(0.25) WITHIN GROUP (ORDER BY problem_count) AS "p25!",
                PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY problem_count) AS "median!",
                PERCENTILE_CONT(0.75) WITHIN GROUP (ORDER BY problem_count) AS "p75!",
                PERCENTILE_CONT(0.90) WITHIN GROUP (ORDER BY problem_count) AS "p90!",
                MAX(problem_count) AS "max!",
                AVG(problem_count)::float8 AS "mean!"
            FROM logs_problem_set JOIN logs_pdf ON logs_problem_set.pdf_id = logs_pdf.id
            WHERE logs_pdf.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        "#,
        duration.to_interval()
    )
    .fetch_one(pool)
    .await?;

    Ok(box_plot)
}
