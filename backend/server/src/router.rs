use crate::{
    api,
    middleware::{self, auth::authenticate_with_limit, rate_limiting::AuthLimit},
    pdf_generation,
};
use axum::{
    Router,
    http::Request,
    routing::{delete, get, patch, post},
};
use std::time::Duration;
use tower_http::trace::{DefaultOnEos, TraceLayer};
use tracing::Level;

pub fn create_router() -> Router {
    let auth_limit = AuthLimit::new();

    let user_routes = Router::new()
        .route("/translations/{lang}", get(api::public::get_translations))
        .route("/{lang}/course", get(api::public::get_course_list))
        .route(
            "/{lang}/course/{course}",
            get(api::public::get_chapters_and_topics_for_course),
        )
        .route(
            "/{lang}/problems",
            post(api::public::get_problems_for_topics),
        )
        .route("/test/{arg}", get(api::testing::test_api))
        .layer(tower_governor::GovernorLayer::new(
            middleware::rate_limiting::json_limit(),
        ));
    let pdf_routes = Router::new()
        .route("/pdf", post(pdf_generation::generate_pdf_from_http))
        .route("/pdf/example", get(pdf_generation::generate_example_pdf))
        .layer(tower_governor::GovernorLayer::new(
            middleware::rate_limiting::pdf_limit(),
        ));

    let protected_routes = Router::new()
        .route("/edit", get(middleware::auth::protected))
        .route("/edit/login", get(middleware::auth::login))
        //.route("/create/{user}/{pass}", post(middleware::auth::create_user))
        // ========================================
        //      PROBLEMS
        // ========================================
        .route("/edit/problem", get(api::editing::problems::get_problems))
        .route(
            "/edit/problem/from_topic",
            post(api::editing::problems::get_problems_from_topic_id),
        )
        .route(
            "/edit/problem",
            post(api::editing::problems::create_problem),
        )
        .route(
            "/edit/problem",
            patch(api::editing::problems::update_problem),
        )
        .route(
            "/edit/problem",
            delete(api::editing::problems::delete_problem),
        )
        // ========================================
        //      TOPICS
        // ========================================
        .route("/edit/topic", get(api::editing::topics::get_topics))
        .route(
            "/edit/topic/ids",
            post(api::editing::topics::get_topics_from_ids),
        )
        .route("/edit/topic", post(api::editing::topics::create_topic))
        .route("/edit/topic", patch(api::editing::topics::update_topic))
        .route("/edit/topic", delete(api::editing::topics::delete_topic))
        // ========================================
        //      CHAPTERS
        // ========================================
        .route("/edit/chapter", get(api::editing::chapters::get_chapters))
        .route(
            "/edit/chapter/ids",
            post(api::editing::chapters::get_chapters_from_ids),
        )
        .route(
            "/edit/chapter",
            post(api::editing::chapters::create_chapter),
        )
        .route(
            "/edit/chapter",
            patch(api::editing::chapters::update_chapter),
        )
        .route(
            "/edit/chapter",
            delete(api::editing::chapters::delete_chapter),
        )
        // ========================================
        //      COURSES
        // ========================================
        .route("/edit/course", get(api::editing::courses::get_courses))
        .route(
            "/edit/course/ids",
            post(api::editing::courses::get_courses_from_ids),
        )
        .route("/edit/course", post(api::editing::courses::create_course))
        .route("/edit/course", patch(api::editing::courses::update_course))
        .route("/edit/course", delete(api::editing::courses::delete_course))
        // ========================================
        //      PREFIXES
        // ========================================
        .route("/edit/prefix", get(api::editing::prefixes::get_prefixes))
        .route("/edit/prefix", post(api::editing::prefixes::create_prefix))
        .route("/edit/prefix", patch(api::editing::prefixes::update_prefix))
        .route(
            "/edit/prefix",
            delete(api::editing::prefixes::delete_prefix),
        )
        .route(
            "/edit/prefix/id/{prefix_id}",
            get(api::editing::prefixes::get_prefix_from_id),
        )
        .layer(axum::middleware::from_fn_with_state(
            auth_limit.clone(),
            authenticate_with_limit,
        ))
        .with_state(auth_limit);

    Router::new()
        .merge(user_routes)
        .merge(pdf_routes)
        .merge(protected_routes)
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
                )
                .on_eos(DefaultOnEos::new().level(Level::TRACE)),
        )
}
