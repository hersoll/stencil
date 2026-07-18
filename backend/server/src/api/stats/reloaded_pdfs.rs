use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use db::logging::stats;
use serde_json::json;
use types::errors::ApiError;

use crate::api::stats::DurationPath;

pub async fn most_reloaded_topics(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let leaderboard =
        stats::reloaded_pdfs::most_reloaded_topics(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(leaderboard))))
}

pub async fn most_changed_fields(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let leaderboard =
        stats::reloaded_pdfs::most_changed_fields(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(leaderboard))))
}
