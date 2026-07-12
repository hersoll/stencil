use crate::{
    api::{
        self,
        stats::{api_counts, box_plots, leaderboards, pdf_attributes, problem_set_attributes},
    },
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

    let auth_routes = Router::new()
        .route("/edit", get(middleware::auth::protected))
        .route("/login", get(middleware::auth::login));
    //.route("/create/{user}/{pass}", post(middleware::auth::create_user))

    let edit_problem_routes = Router::new()
        .route("/", get(api::editing::problems::get_problems))
        .route("/", post(api::editing::problems::create_problem))
        .route("/", patch(api::editing::problems::update_problem))
        .route("/", delete(api::editing::problems::delete_problem))
        .route(
            "/from_topic",
            post(api::editing::problems::get_problems_from_topic_id),
        );
    let edit_topic_routes = Router::new()
        .route("/", get(api::editing::topics::get_topics))
        .route("/", post(api::editing::topics::create_topic))
        .route("/", patch(api::editing::topics::update_topic))
        .route("/", delete(api::editing::topics::delete_topic))
        .route("/ids", post(api::editing::topics::get_topics_from_ids));
    let edit_chapter_routes = Router::new()
        .route("/", get(api::editing::chapters::get_chapters))
        .route("/", post(api::editing::chapters::create_chapter))
        .route("/", patch(api::editing::chapters::update_chapter))
        .route("/", delete(api::editing::chapters::delete_chapter))
        .route("/ids", post(api::editing::chapters::get_chapters_from_ids));
    let edit_course_routes = Router::new()
        .route("/", get(api::editing::courses::get_courses))
        .route("/", post(api::editing::courses::create_course))
        .route("/", patch(api::editing::courses::update_course))
        .route("/", delete(api::editing::courses::delete_course))
        .route("/ids", post(api::editing::courses::get_courses_from_ids));
    let edit_prefix_routes = Router::new()
        .route("/", get(api::editing::prefixes::get_prefixes))
        .route("/", post(api::editing::prefixes::create_prefix))
        .route("/", patch(api::editing::prefixes::update_prefix))
        .route("/", delete(api::editing::prefixes::delete_prefix))
        .route(
            "/id/{prefix_id}",
            get(api::editing::prefixes::get_prefix_from_id),
        );

    let edit_routes = Router::new()
        .nest("/edit/problem", edit_problem_routes)
        .nest("/edit/topic", edit_topic_routes)
        .nest("/edit/chapter", edit_chapter_routes)
        .nest("/edit/course", edit_course_routes)
        .nest("/edit/prefix", edit_prefix_routes);

    let leaderboard_routes = Router::new()
        .route("/topics/{duration}", get(leaderboards::most_used_topics))
        .route(
            "/exclusions/{duration}",
            get(leaderboards::most_excluded_problems),
        );

    let box_plot_routes = Router::new()
        .route("/renders/{duration}", get(box_plots::render_times))
        .route("/topics/{duration}", get(box_plots::topics_per_set))
        .route("/exclusions/{duration}", get(box_plots::exclusions_per_set))
        .route(
            "/problem_count/{duration}",
            get(box_plots::problem_count_per_set),
        );

    let stats_routes = Router::new()
        .nest("/leaderboard", leaderboard_routes)
        .nest("/boxplots", box_plot_routes)
        .route("/pdf", get(api_counts::get_pdf_count))
        .route("/pdf/{duration}", get(api_counts::get_pdf_timeline))
        .route(
            "/pdf/{attribute}/{duration}",
            get(pdf_attributes::get_pdf_attribute),
        )
        .route(
            "/set/{attribute}/{duration}",
            get(problem_set_attributes::get_problem_set_attribute),
        )
        .route("/lang/{duration}", get(api_counts::get_language_count))
        .route("/course/{duration}", get(api_counts::get_course_count));

    let protected_routes = Router::new()
        .nest("/stats", stats_routes)
        .merge(auth_routes)
        .merge(edit_routes)
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
