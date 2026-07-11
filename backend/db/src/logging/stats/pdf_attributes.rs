use anyhow::Result;

use crate::logging::stats::{AggregationDuration, ValueCount};

// All functions in this module are identical apart from the field they query.
// We could do this dynamically but then we lose type safety (what happens if I rename a DB column?).

pub async fn get_title_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT has_title as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY has_title;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_subtitle_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT has_subtitle as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY has_subtitle;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_name_field_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT has_name_field as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY has_name_field;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_font_size_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT font_size as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY font_size;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_answer_column_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT answer_columns as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY answer_columns;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_lang_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT lang as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY lang;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_write_solutions_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT write_solutions as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY write_solutions;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_color_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT color as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY color;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_paper_size_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT paper_size as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY paper_size;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_x_margin_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT x_margin as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY x_margin;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_y_margin_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT y_margin as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY y_margin;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_par_spacing_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT par_spacing as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY par_spacing;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_max_prefix_group_count(duration: AggregationDuration) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT max_prefix_group as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY max_prefix_group;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}

pub async fn get_page_break_before_answer_count(
    duration: AggregationDuration,
) -> Result<Vec<ValueCount>> {
    let pool = crate::get_pool();
    let counts = sqlx::query_as!(
        ValueCount,
        r#"SELECT page_break_before_answers as value, COUNT(*) as "count!" 
        FROM logs_pdf 
        WHERE created_at >= (NOW() AT TIME ZONE 'utc') - $1::interval
        GROUP BY page_break_before_answers;"#,
        duration.to_interval()
    )
    .fetch_all(pool)
    .await?;

    Ok(counts)
}
