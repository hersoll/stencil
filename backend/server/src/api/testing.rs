use axum::{extract::Path, http::StatusCode, response::IntoResponse};
use types::errors::ApiError;

pub async fn test_api(Path(arg): Path<i32>) -> Result<impl IntoResponse, ApiError> {
    cfg_select! {
        feature = "docker" => Ok((
            StatusCode::OK,
            format!("Currently not in use! You sent arg {arg}"),
        )),
        _ => do_the_test(arg).await
    }
}

async fn do_the_test(arg: i32) -> Result<impl IntoResponse, ApiError> {
    Ok((
        StatusCode::OK,
        format!("Currently not in use! You sent arg {arg}"),
    ))
}
