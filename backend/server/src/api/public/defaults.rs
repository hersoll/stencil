use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use serde_json::json;
use types::{
    errors::ApiError,
    pdf::{DocumentOptions, ProblemOptions, QuestionSetFormattingOptions},
};

#[derive(Serialize)]
struct Defaults {
    problem_options: ProblemOptions,
    formatting_options: QuestionSetFormattingOptions,
    document_options: DocumentOptions,
}

/// Returns all of the default values and relevant limits
///
/// This ensures that we only have to change values in one place instead of trying to keep the
/// backend and frontend in sync.
pub async fn get_defaults() -> Result<impl IntoResponse, ApiError> {
    let defaults = Defaults {
        problem_options: ProblemOptions::default(),
        formatting_options: QuestionSetFormattingOptions::default(),
        document_options: DocumentOptions::default(),
    };

    Ok((
        StatusCode::OK,
        [
            ("Cache-Control", "public, max-age=3600"), // Cache 1 hour
            ("Content-Type", "application/json"),
        ],
        Json(json!(defaults)),
    ))
}
