use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde::Deserialize;
use serde_json::json;
use types::errors::ApiError;

use crate::api::stats::DurationPath;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemSetAttribute {
    Columns,
    Heading,
    Spacing,
    PageBreak,
    StartingDifficulty,
    EndingDifficulty,
}

/// To keep the router clean, we have one endpoint which is dynamic over attribute and duration.
pub async fn get_problem_set_attribute(
    Path((attribute, duration_path)): Path<(ProblemSetAttribute, DurationPath)>,
) -> Result<impl IntoResponse, ApiError> {
    use ProblemSetAttribute::*;
    use db::logging::stats;
    let duration = duration_path.as_duration();
    let count = match attribute {
        Columns => stats::get_set_column_count(duration).await?,
        Heading => stats::get_set_heading_count(duration).await?,
        Spacing => stats::get_set_spacing_count(duration).await?,
        PageBreak => stats::get_set_page_break_count(duration).await?,
        StartingDifficulty => stats::get_set_starting_difficulty_count(duration).await?,
        EndingDifficulty => stats::get_set_ending_difficulty_count(duration).await?,
    };
    Ok((StatusCode::OK, Json(json!(count))))
}
