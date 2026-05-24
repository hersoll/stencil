use crate::{
    middleware::{self, auth::authenticate},
    pdf_generation, text_endpoints,
};
use axum::{
    Router,
    http::Request,
    routing::{delete, get, patch, post},
};
use std::time::Duration;
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
        .route("/{lang}/problems", post(db::api::get_problems));
    let pdf_routes = Router::new()
        .route("/pdf", post(pdf_generation::generate_pdf_from_http))
        .route("/pdf/example", get(pdf_generation::generate_example_pdf));

    // Only included in dev build
    #[cfg(not(feature = "docker"))]
    let protected_routes = Router::new()
        .route("/edit/login", get(middleware::auth::login))
        .route("/edit", get(text_endpoints::protected))
        .route("/create/{user}/{pass}", post(middleware::auth::create_user))
        // ========================================
        //      PROBLEMS
        // ========================================
        .route("/edit/problem", get(db::edit::get_problems))
        .route("/edit/problem/ids", post(db::edit::get_problems_from_ids))
        .route("/edit/problem", post(db::edit::create_problem))
        .route("/edit/problem", patch(db::edit::update_problem))
        .route("/edit/problem", delete(db::edit::delete_problem))
        .route(
            "/edit/problem/{id}/topics",
            get(db::edit::get_topics_from_problem),
        )
        // ========================================
        //      TOPICS
        // ========================================
        .route("/edit/topic", get(db::edit::get_topics))
        .route("/edit/topic/ids", post(db::edit::get_topics_from_ids))
        .route("/edit/topic", post(db::edit::create_topic))
        .route("/edit/topic", patch(db::edit::update_topic))
        .route("/edit/topic", delete(db::edit::delete_topic))
        // Probably not in use, remove if nothing has broken
        // .route(
        //     "/edit/topic/{id}/problems",
        //     get(db::edit::get_problems_from_topic),
        // )
        .route(
            "/edit/topic/{id}/chapters",
            get(db::edit::get_chapters_from_topic),
        )
        // ========================================
        //      CHAPTERS
        // ========================================
        .route("/edit/chapter", get(db::edit::get_chapters))
        .route("/edit/chapter/ids", post(db::edit::get_chapters_from_ids))
        .route("/edit/chapter", post(db::edit::create_chapter))
        .route("/edit/chapter", patch(db::edit::update_chapter))
        .route("/edit/chapter", delete(db::edit::delete_chapter))
        .route(
            "/edit/chapter/{id}/topics",
            get(db::edit::get_topics_from_chapter),
        )
        .route(
            "/edit/chapter/{id}/courses",
            get(db::edit::get_courses_from_chapter),
        )
        // ========================================
        //      COURSES
        // ========================================
        .route("/edit/course", get(db::edit::get_courses))
        .route("/edit/course/ids", post(db::edit::get_courses_from_ids))
        .route("/edit/course", post(db::edit::create_course))
        .route("/edit/course", patch(db::edit::update_course))
        .route("/edit/course", delete(db::edit::delete_course))
        .route(
            "/edit/course/{id}/chapters",
            get(db::edit::get_chapters_from_course),
        )
        // ========================================
        //      PREFIXES
        // ========================================
        .route("/edit/prefix", get(db::edit::get_prefixes))
        .route("/edit/prefix", post(db::edit::create_prefix))
        .route("/edit/prefix", patch(db::edit::update_prefix))
        .route("/edit/prefix", delete(db::edit::delete_prefix))
        .route(
            "/edit/prefix/id/{prefix_id}",
            get(db::edit::get_prefix_from_id),
        )
        .layer(axum::middleware::from_fn(authenticate));

    #[cfg(feature = "docker")]
    let api_router = Router::new()
        .merge(standard_routes)
        .layer(tower_governor::GovernorLayer::new(
            middleware::rate_limiting::json_limit(),
        ))
        .merge(pdf_routes)
        .layer(tower_governor::GovernorLayer::new(
            middleware::rate_limiting::pdf_limit(),
        ));

    #[cfg(not(feature = "docker"))]
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
