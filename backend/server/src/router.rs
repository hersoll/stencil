use crate::{
    middleware::{self, auth::authenticate},
    pdf_generation,
};
use axum::{
    Router,
    http::Request,
    routing::{delete, get, patch, post},
};
use std::time::Duration;
use tower_http::trace::TraceLayer;

pub fn create_router() -> Router {
    let user_routes = Router::new()
        .route(
            "/translations/{lang}",
            get(db::public_api::get_translations),
        )
        .route("/{lang}/course", get(db::public_api::get_course_list))
        .route(
            "/{lang}/course/{course}",
            get(db::public_api::get_chapters_and_topics_for_course),
        )
        .route(
            "/{lang}/problems",
            post(db::public_api::get_problems_for_topics),
        );
    let pdf_routes = Router::new()
        .route("/pdf", post(pdf_generation::generate_pdf_from_http))
        .route("/pdf/example", get(pdf_generation::generate_example_pdf));

    // Only included in dev build
    #[cfg(not(feature = "docker"))]
    let protected_routes = Router::new()
        .route("/edit/login", get(middleware::auth::login))
        .route("/create/{user}/{pass}", post(middleware::auth::create_user))
        // ========================================
        //      PROBLEMS
        // ========================================
        .route("/edit/problem", get(db::editing_api::get_problems))
        .route(
            "/edit/problem/ids",
            post(db::editing_api::get_problems_from_ids),
        )
        .route("/edit/problem", post(db::editing_api::create_problem))
        .route("/edit/problem", patch(db::editing_api::update_problem))
        .route("/edit/problem", delete(db::editing_api::delete_problem))
        // ========================================
        //      TOPICS
        // ========================================
        .route("/edit/topic", get(db::editing_api::get_topics))
        .route(
            "/edit/topic/ids",
            post(db::editing_api::get_topics_from_ids),
        )
        .route("/edit/topic", post(db::editing_api::create_topic))
        .route("/edit/topic", patch(db::editing_api::update_topic))
        .route("/edit/topic", delete(db::editing_api::delete_topic))
        // Probably not in use, remove if nothing has broken
        // .route(
        //     "/edit/topic/{id}/problems",
        //     get(db::edit::get_problems_from_topic),
        // )
        .route(
            "/edit/topic/{id}/chapters",
            get(db::editing_api::get_chapters_from_topic),
        )
        // ========================================
        //      CHAPTERS
        // ========================================
        .route("/edit/chapter", get(db::editing_api::get_chapters))
        .route(
            "/edit/chapter/ids",
            post(db::editing_api::get_chapters_from_ids),
        )
        .route("/edit/chapter", post(db::editing_api::create_chapter))
        .route("/edit/chapter", patch(db::editing_api::update_chapter))
        .route("/edit/chapter", delete(db::editing_api::delete_chapter))
        .route(
            "/edit/chapter/{id}/topics",
            get(db::editing_api::get_topics_from_chapter),
        )
        .route(
            "/edit/chapter/{id}/courses",
            get(db::editing_api::get_courses_from_chapter),
        )
        // ========================================
        //      COURSES
        // ========================================
        .route("/edit/course", get(db::editing_api::get_courses))
        .route(
            "/edit/course/ids",
            post(db::editing_api::get_courses_from_ids),
        )
        .route("/edit/course", post(db::editing_api::create_course))
        .route("/edit/course", patch(db::editing_api::update_course))
        .route("/edit/course", delete(db::editing_api::delete_course))
        .route(
            "/edit/course/{id}/chapters",
            get(db::editing_api::get_chapters_from_course),
        )
        // ========================================
        //      PREFIXES
        // ========================================
        .route("/edit/prefix", get(db::editing_api::get_prefixes))
        .route("/edit/prefix", post(db::editing_api::create_prefix))
        .route("/edit/prefix", patch(db::editing_api::update_prefix))
        .route("/edit/prefix", delete(db::editing_api::delete_prefix))
        .route(
            "/edit/prefix/id/{prefix_id}",
            get(db::editing_api::get_prefix_from_id),
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
        .merge(user_routes)
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
