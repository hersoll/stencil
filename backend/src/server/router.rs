use crate::{db, pdf_generation, server::middleware, text_endpoints};
use axum::{http::Request, routing::get, Router};
use std::time::Duration;
use tower_governor::GovernorLayer;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(text_endpoints::welcome))
        .route("/help", get(text_endpoints::help))
        .route("/translations/{lang}", get(db::api::get_translation))
        .route("/{lang}/course/{course}", get(db::api::get_course))
        .route(
            "/{lang}/course/{course}/{chapter}",
            get(db::api::get_chapter),
        )
        .route(
            "/{lang}/course/{course}/{chapter}/{topic}",
            get(db::api::get_topic),
        )
        .layer(GovernorLayer::new(middleware::rate_limiting::json_limit()))
        .route("/pdf", get(pdf_generation::generate_pdf_from_http))
        .route("/pdf/example", get(pdf_generation::generate_example_pdf))
        .layer(middleware::cors::create_cors_layer())
        // Annoying type signature, don't try to extract to its own function...
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    tracing::info_span!(
                        "http",
                        method = ?request.method(),
                        uri = ?request.uri(),
                    )
                })
                .on_response(
                    |response: &axum::response::Response,
                     latency: Duration,
                     _span: &tracing::Span| {
                        tracing::info!(
                            status = response.status().as_u16(),
                            latency = ?latency,
                            "response"
                        );
                    },
                ),
        )
}
