use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use db::logging;
use serde_json::json;
use types::errors::ApiError;

use crate::api::parse_language;

/// Returns all of text on the web page in the specified language
///
/// The data is returned in the form of a [`HashMap`], where the keys are identifiers
/// for each string and the values are text in the required language
pub async fn get_translations(
    Path(lang_code): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let lang = parse_language(&lang_code)?;

    // Only log during production (or specific flag) to not mess up the stats
    if cfg!(feature = "docker") || std::env::args().any(|x| x == "log") {
        logging::log_language(lang).await?;
    }

    let translations = db::i18n::get_i18n_for_web(&lang).await?;

    Ok((
        StatusCode::OK,
        [
            ("Cache-Control", "public, max-age=3600"), // Cache 1 hour
            ("Content-Type", "application/json"),
        ],
        Json(json!(translations)),
    ))
}
