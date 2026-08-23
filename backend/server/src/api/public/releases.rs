use crate::github::{GITHUB_RELEASES, GithubRelease};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use types::errors::ApiError;

/// Get a list of all the releases
pub async fn get_releases() -> Result<impl IntoResponse, ApiError> {
    let releases: Vec<GithubRelease> = {
        let guard = GITHUB_RELEASES
            .read()
            .map_err(|e| ApiError::Other(anyhow::anyhow!(e.to_string())))?;
        guard.clone()
    };

    Ok((
        StatusCode::OK,
        [
            ("Cache-Control", "public, max-age=3600"), // Cache 1 hour
            ("Content-Type", "application/json"),
        ],
        Json(json!(releases)),
    ))
}

pub async fn get_latest_tag() -> Result<impl IntoResponse, ApiError> {
    let tag_name = GITHUB_RELEASES
        .read()
        .map_err(|e| ApiError::Other(anyhow::anyhow!(e.to_string())))?
        .first()
        .map(|release| release.tag_name.clone());

    Ok(Json(tag_name))
}
