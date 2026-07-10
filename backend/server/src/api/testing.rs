use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use serde_json::json;
use types::errors::ApiError;

pub async fn test_api(Path(arg): Path<u8>) -> Result<impl IntoResponse, ApiError> {
    cfg_select! {
        feature = "docker" => Ok((
            StatusCode::OK,
            format!("Currently not in use! You sent arg {arg}"),
        )),
        _ => do_the_test(arg).await
    }
}

async fn do_the_test(_arg: u8) -> Result<impl IntoResponse, ApiError> {
    let courses = db::logging::stats::get_pdf_count_daily_for_week().await?;
    Ok((StatusCode::OK, Json(json!(courses))))
}
