use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub fn create_cors_layer() -> CorsLayer {
    let frontend_port = if cfg!(feature = "docker") {
        "5172"
    } else {
        "5173"
    };
    let frontend_url = format!("http://localhost:{frontend_port}");

    info!("Allowing connections from {frontend_url}.");

    CorsLayer::new()
        .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}
