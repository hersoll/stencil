use axum::{
    Router,
    http::{Method, Request},
    routing::get,
};
use std::time::Duration;
use stencil::{db, pdf_generation, registry, text_endpoints};
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
    match db::init_database().await {
        Ok(_) => info!("Finished initializing db!"),
        Err(_) => error!("Failed to initialize db!"),
    }

    info!("Loading problems to registry...");
    match registry::load_problem_data().await {
        Ok(_) => info!("Problems loaded!"),
        Err(_) => error!("Failed to load problems from registry!"),
    }
    info!("Loading prefixes to registry...");
    match registry::load_prefix_data().await {
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
        .route("/", get(text_endpoints::welcome))
        .route("/help", get(text_endpoints::help))
        .route("/pdf", get(pdf_generation::generate_pdf_from_http))
        .route("/pdf/example", get(pdf_generation::generate_example_pdf))
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
