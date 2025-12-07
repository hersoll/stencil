use axum::{
    Router,
    http::{Method, Request, StatusCode},
    routing::get,
};
use std::time::Duration;
use stencil::builders::pdf_builder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{error, info, warn};
use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Set up tracing
    let default_filter = if cfg!(debug_assertions) {
        "debug,stencil=debug,sqlx=warn"
    } else {
        "info,stencil=info,sqlx=warn"
    };
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| default_filter.into()),
        )
        .with(tracing_subscriber::fmt::layer().with_target(true))
        .init();

    info!("Starting server...");

    // Load IP and port from env
    let default_host = "127.0.0.1";
    let default_port = "3000";
    let host = std::env::var("HOST").unwrap_or_else(|_| {
        warn!("HOST not found in .env, using {default_host}.");
        default_host.to_string()
    });
    let port = std::env::var("PORT").unwrap_or_else(|_| {
        warn!("PORT not found in .env, using {default_port}.");
        default_port.to_string()
    });
    let addr = format!("{}:{}", host, port);

    info!("Initializing db...");
    match stencil::db::init_database().await {
        Ok(_) => info!("Finished initializing db!"),
        Err(_) => error!("Failed to initialize db!"),
    }

    info!("Loading problems to registry...");
    match stencil::load_problem_data().await {
        Ok(_) => info!("Problems loaded!"),
        Err(_) => error!("Failed to load problems from registry!"),
    }
    info!("Loading prefixes to registry...");
    match stencil::load_prefix_data().await {
        Ok(_) => info!("Prefixes loaded!"),
        Err(_) => error!("Failed to load prefixes from registry!"),
    }

    #[cfg(feature = "docker")]
    let frontend_port = "5172";
    #[cfg(not(feature = "docker"))]
    let frontend_port = "5173";

    let frontend_url = format!("http://localhost:{frontend_port}");
    let cors = create_cors_layer(&frontend_url);
    info!("Allowing connections from {frontend_url}.");

    let app = create_router(cors);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    // Printing empty line to clearly show that setup is finished
    // and the server is listening
    println!();
    info!("Listening on {}...", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

fn create_router(cors_layer: CorsLayer) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/error", get(error_test))
        .route("/pdf", get(pdf_builder::send_pdf))
        .layer(cors_layer)
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

fn create_cors_layer(allowed_url: &String) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(allowed_url.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
}

async fn hello_world() -> String {
    String::from(
        "Hello from the \"/\" path! Did you mean to hit the \"/api\"?
            Try hitting the \"/error\" path to get a funny HTTP error!
        ",
    )
}

async fn error_test() -> Result<String, StatusCode> {
    error!("This is an error code!");
    Err(StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
}
