use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use db::logging::stats;
use serde_json::json;
use types::errors::ApiError;

use crate::api::stats::DurationPath;

pub async fn render_times(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let box_plot = stats::box_plots::render_times(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(box_plot))))
}

pub async fn topics_per_set(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let box_plot = stats::box_plots::topics_per_set(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(box_plot))))
}

pub async fn exclusions_per_set(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let box_plot = stats::box_plots::exclusions_per_set(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(box_plot))))
}

pub async fn problem_count_per_set(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let box_plot = stats::box_plots::problem_count_per_set(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(box_plot))))
}
