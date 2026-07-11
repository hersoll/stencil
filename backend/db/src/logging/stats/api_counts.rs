use anyhow::Result;

use crate::logging::stats::{AggregationDuration, TimeLineCount, ValueCount};

pub async fn get_language_api_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT lang as value, COUNT(*) as "count!" 
        FROM logs_lang 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY lang;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_course_api_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT courses.desc_sv as value, COUNT(*) as "count!" 
        FROM logs_course INNER JOIN courses ON logs_course.course_id = courses.id
        WHERE logs_course.created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY courses.desc_sv;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

/// Returns the number of PDFs that has been generated, ever
pub async fn get_pdf_count_all_time() -> Result<i64> {
    let pool = crate::get_pool();
    let record = sqlx::query!(
        r#"SELECT COUNT(*) as "count!" 
        FROM logs_pdf 
        "#,
    )
    .fetch_one(pool)
    .await?;

    Ok(record.count)
}

/// Returns the PDF count for every hour in the last 24 hours
pub async fn get_pdf_count_hourly_for_day() -> Result<Vec<TimeLineCount>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        TimeLineCount,
        r#"
        WITH hours AS (
            SELECT generate_series(
                date_trunc('hour', now() AT TIME ZONE 'utc') - interval '23 hours',
                date_trunc('hour', now() AT TIME ZONE 'utc'),
                interval '1 hour'
            ) AS hour
        )
        SELECT
            h.hour as "time!",
            COALESCE(c.count, 0)::bigint as "count!"
        FROM hours h
        LEFT JOIN (
            SELECT
                date_trunc('hour', created_at) AS hour,
                COUNT(*) AS count
            FROM logs_pdf
            WHERE created_at >= (now() AT TIME ZONE 'utc') - interval '24 hours'
            GROUP BY 1
        ) c ON h.hour = c.hour
        ORDER BY h.hour;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_pdf_count_daily_for_week() -> Result<Vec<TimeLineCount>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        TimeLineCount,
        r#"
        WITH days AS (
            SELECT generate_series(
                date_trunc('day', (now() AT TIME ZONE 'utc')) - interval '6 days',
                date_trunc('day', (now() AT TIME ZONE 'utc')),
                interval '1 day'
            ) AS day
        )
        SELECT
            d.day as "time!",
            COALESCE(c.count, 0)::bigint AS "count!"
        FROM days d
        LEFT JOIN (
            SELECT
                date_trunc('day', created_at) AS day,
                COUNT(*) AS count
            FROM logs_pdf
            WHERE created_at >= (now() AT TIME ZONE 'utc') - interval '7 days'
            GROUP BY 1
        ) c ON c.day = d.day
        ORDER BY d.day;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_pdf_count_daily_for_month() -> Result<Vec<TimeLineCount>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        TimeLineCount,
        r#"
        WITH days AS (
            SELECT generate_series(
                date_trunc('day', (now() AT TIME ZONE 'utc')) - interval '29 days',
                date_trunc('day', (now() AT TIME ZONE 'utc')),
                interval '1 day'
            ) AS day
        )
        SELECT
            d.day as "time!",
            COALESCE(c.count, 0)::bigint AS "count!"
        FROM days d
        LEFT JOIN (
            SELECT
                date_trunc('day', created_at) AS day,
                COUNT(*) AS count
            FROM logs_pdf
            WHERE created_at >= (now() AT TIME ZONE 'utc') - interval '30 days'
            GROUP BY 1
        ) c USING (day)
        ORDER BY d.day;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_pdf_count_weekly_for_three_months() -> Result<Vec<TimeLineCount>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        TimeLineCount,
        r#"
        WITH weeks AS (
            SELECT generate_series(
                date_trunc('week', (now() AT TIME ZONE 'utc')) - interval '12 weeks',
                date_trunc('week', (now() AT TIME ZONE 'utc')),
                interval '1 week'
            ) AS week_start
        )
        SELECT
            w.week_start AS "time!",
            COALESCE(c.count, 0)::bigint AS "count!"
        FROM weeks w
        LEFT JOIN (
            SELECT
                date_trunc('week', created_at) AS week_start,
                COUNT(*) AS count
            FROM logs_pdf
            WHERE created_at >= (now() AT TIME ZONE 'utc') - interval '3 months'
            GROUP BY 1
        ) c USING (week_start)
        ORDER BY w.week_start;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_pdf_count_weekly_for_year() -> Result<Vec<TimeLineCount>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        TimeLineCount,
        r#"
        WITH weeks AS (
            SELECT generate_series(
                date_trunc('week', (now() AT TIME ZONE 'utc')) - interval '52 weeks',
                date_trunc('week', (now() AT TIME ZONE 'utc')),
                interval '1 week'
            ) AS week_start
        )
        SELECT
            w.week_start AS "time!",
            COALESCE(c.count, 0)::bigint AS "count!"
        FROM weeks w
        LEFT JOIN (
            SELECT
                date_trunc('week', created_at) AS week_start,
                COUNT(*) AS count
            FROM logs_pdf
            WHERE created_at >= (now() AT TIME ZONE 'utc') - interval '1 year'
            GROUP BY 1
        ) c USING (week_start)
        ORDER BY w.week_start;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_pdf_count_monthly_for_all_time() -> Result<Vec<TimeLineCount>> {
    let pool = crate::get_pool();
    let rows = sqlx::query_as!(
        TimeLineCount,
        r#"
        WITH months AS (
            SELECT generate_series(
                '2026-07-01 00:00:00'::timestamp,
                date_trunc('month', (now() AT TIME ZONE 'utc')),
                interval '1 month'
            ) AS month_start
        )
        SELECT
            m.month_start AS "time!",
            COALESCE(c.count, 0)::bigint AS "count!"
        FROM months m
        LEFT JOIN (
            SELECT
                date_trunc('month', created_at) AS month_start,
                COUNT(*) AS count
            FROM logs_pdf
            WHERE created_at >= '2026-07-01 00:00:00'::timestamp 
            GROUP BY 1
        ) c USING (month_start)
        ORDER BY m.month_start;
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}
