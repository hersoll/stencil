use axum::http::{HeaderName, Method};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

pub fn create_cors_layer() -> CorsLayer {
    let frontend_url = std::env::var("FRONTEND_URL").unwrap_or("http://localhost:5173".to_string());

    info!("Allowing connections from {frontend_url}.");

    CorsLayer::new()
        .allow_origin(frontend_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .expose_headers([HeaderName::from_static("x-pdf-id")])
}
