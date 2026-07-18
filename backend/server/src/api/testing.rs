use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};
use db::logging::stats::AggregationDuration;
use serde_json::json;
use types::errors::ApiError;

pub async fn test_api(Path(arg): Path<u16>) -> Result<impl IntoResponse, ApiError> {
    cfg_select! {
        feature = "docker" => Ok((
            StatusCode::OK,
            format!("Currently not in use! You sent arg {arg}"),
        )),
        _ => do_the_test(arg).await
    }
}

async fn do_the_test(arg: u16) -> Result<impl IntoResponse, ApiError> {
    let data =
        db::logging::stats::reloaded_pdfs::most_changed_fields(AggregationDuration::Days(arg))
            .await?;
    Ok((StatusCode::OK, Json(json!(data))))
}
