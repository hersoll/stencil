use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use db::logging::stats;
use serde_json::json;
use types::errors::ApiError;

use crate::api::stats::DurationPath;

pub async fn get_language_count(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_api_count(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

pub async fn get_course_count(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_api_count(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

pub async fn get_pdf_count() -> Result<impl IntoResponse, ApiError> {
    let count = stats::get_pdf_count_all_time().await?;
    Ok((StatusCode::OK, Json(json!(count))))
}

pub async fn get_pdf_timeline(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    use DurationPath::*;
    let count = match duration_path {
        Day => stats::get_pdf_count_hourly_for_day().await?,
        Week => stats::get_pdf_count_daily_for_week().await?,
        Month => stats::get_pdf_count_daily_for_month().await?,
        ThreeMonths => stats::get_pdf_count_weekly_for_three_months().await?,
        Year => stats::get_pdf_count_weekly_for_year().await?,
        All => stats::get_pdf_count_monthly_for_all_time().await?,
    };
    Ok((StatusCode::OK, Json(json!(count))))
}
