use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use db::logging::stats;
use serde_json::json;
use types::errors::ApiError;

use crate::api::stats::DurationPath;

pub async fn most_used_topics(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let leaderboard = stats::leaderboards::most_used_topics(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(leaderboard))))
}

pub async fn most_excluded_problems(
    Path(duration_path): Path<DurationPath>,
) -> Result<impl IntoResponse, ApiError> {
    let leaderboard =
        stats::leaderboards::most_excluded_problems(duration_path.as_duration()).await?;
    Ok((StatusCode::OK, Json(json!(leaderboard))))
}
