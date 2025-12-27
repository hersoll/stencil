use crate::{
    db, pdf_generation,
    server::middleware::{self, auth::authenticate, ip_restriction::restrict_ip},
    text_endpoints,
};
use axum::{
    Router,
    http::Request,
    routing::{delete, get, patch, post},
};
use std::time::Duration;
use tower_governor::GovernorLayer;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    let standard_routes = Router::new()
        .route("/", get(text_endpoints::welcome))
        .route("/help", get(text_endpoints::help))
        .route("/translations/{lang}", get(db::api::get_translation))
        .route("/{lang}/course", get(db::api::get_courses))
        .route("/{lang}/course/{course}", get(db::api::get_course))
        .route(
            "/{lang}/course/{course}/{chapter}",
            get(db::api::get_chapter),
        )
        //.route("/create/{user}/{pass}", post(middleware::auth::create_user))
        .route("/{lang}/problems", post(db::api::get_problems));
    //.layer(GovernorLayer::new(middleware::rate_limiting::json_limit()))
    let pdf_routes = Router::new()
        .route("/pdf", post(pdf_generation::generate_pdf_from_http))
        .route("/pdf/example", get(pdf_generation::generate_example_pdf));
    //.layer(GovernorLayer::new(middleware::rate_limiting::pdf_limit()))
    let protected_routes = Router::new()
        .route("/edit/login", get(middleware::auth::login))
        .route("/edit", get(text_endpoints::protected))
        .layer(GovernorLayer::new(middleware::rate_limiting::auth_limit()))
        .route("/edit/problem", get(db::edit::get_problems))
        .route("/edit/problem", post(db::edit::create_problem))
        .route("/edit/problem", patch(db::edit::update_problem))
        .route("/edit/problem", delete(db::edit::delete_problem))
        .route("/edit/topic", get(db::edit::get_topics))
        .route("/edit/topic", post(db::edit::create_topic))
        .route("/edit/topic", patch(db::edit::update_topic))
        .route("/edit/topic", delete(db::edit::delete_topic))
        .route("/edit/chapter", get(db::edit::get_chapters))
        .route("/edit/chapter", post(db::edit::create_chapter))
        .route("/edit/chapter", patch(db::edit::update_chapter))
        .route("/edit/chapter", delete(db::edit::delete_chapter))
        .route("/edit/course", get(db::edit::get_courses))
        .route("/edit/course", post(db::edit::create_course))
        .route("/edit/course", patch(db::edit::update_course))
        .route("/edit/course", delete(db::edit::delete_course))
        .route("/edit/prefix", get(db::edit::get_prefixes))
        .route("/edit/prefix", post(db::edit::create_prefix))
        .route("/edit/prefix", patch(db::edit::update_prefix))
        .route("/edit/prefix", delete(db::edit::delete_prefix))
        .layer(GovernorLayer::new(middleware::rate_limiting::json_limit()))
        .layer(axum::middleware::from_fn(authenticate))
        .layer(axum::middleware::from_fn(|req, next| {
            restrict_ip(req, next, vec!["127.0.0.1".parse().unwrap()])
        }));

    let api_router = Router::new()
        .merge(standard_routes)
        .merge(pdf_routes)
        .merge(protected_routes);

    Router::new()
        .nest("/api", api_router)
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
