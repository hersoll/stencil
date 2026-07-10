use axum::{Json, http::StatusCode, response::IntoResponse};
use db::logging::stats::{self, AggregationDuration};
use serde_json::json;
use types::errors::ApiError;

// ####################
//      Languages
// ####################
pub async fn language_for_day() -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_count(AggregationDuration::Hours(24)).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

pub async fn language_for_week() -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_count(AggregationDuration::Days(7)).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

pub async fn language_for_month() -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_count(AggregationDuration::Days(30)).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

pub async fn language_for_three_months() -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_count(AggregationDuration::Days(90)).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

pub async fn language_for_year() -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_count(AggregationDuration::Days(365)).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

pub async fn language_for_all_time() -> Result<impl IntoResponse, ApiError> {
    let lang_counts = stats::get_language_count(AggregationDuration::AllTime).await?;
    Ok((StatusCode::OK, Json(json!(lang_counts))))
}

// ####################
//       Courses
// ####################
pub async fn courses_for_day() -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_count(AggregationDuration::Hours(24)).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

pub async fn courses_for_week() -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_count(AggregationDuration::Days(7)).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

pub async fn courses_for_month() -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_count(AggregationDuration::Days(30)).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

pub async fn courses_for_three_months() -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_count(AggregationDuration::Days(90)).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

pub async fn courses_for_year() -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_count(AggregationDuration::Days(365)).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

pub async fn courses_for_all_time() -> Result<impl IntoResponse, ApiError> {
    let course_counts = stats::get_course_count(AggregationDuration::AllTime).await?;
    Ok((StatusCode::OK, Json(json!(course_counts))))
}

// ####################
//         PDFs
// ####################
pub async fn pdf_hourly_for_day() -> Result<impl IntoResponse, ApiError> {
    let hours = stats::get_pdf_count_hourly_for_day().await?;
    Ok((StatusCode::OK, Json(json!(hours))))
}

pub async fn pdf_daily_for_week() -> Result<impl IntoResponse, ApiError> {
    let days = stats::get_pdf_count_daily_for_week().await?;
    Ok((StatusCode::OK, Json(json!(days))))
}

pub async fn pdf_daily_for_month() -> Result<impl IntoResponse, ApiError> {
    let days = stats::get_pdf_count_daily_for_month().await?;
    Ok((StatusCode::OK, Json(json!(days))))
}

pub async fn pdf_weekly_for_three_months() -> Result<impl IntoResponse, ApiError> {
    let weeks = stats::get_pdf_count_weekly_for_three_months().await?;
    Ok((StatusCode::OK, Json(json!(weeks))))
}

pub async fn pdf_weekly_for_year() -> Result<impl IntoResponse, ApiError> {
    let weeks = stats::get_pdf_count_weekly_for_year().await?;
    Ok((StatusCode::OK, Json(json!(weeks))))
}

pub async fn pdf_all_time() -> Result<impl IntoResponse, ApiError> {
    let count = stats::get_pdf_count_all_time().await?;
    Ok((StatusCode::OK, Json(json!(count))))
}
