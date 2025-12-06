use stencil::pdf_generation;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber;

use axum::{Router, http::Method, routing::get};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    let cors = CorsLayer::new()
        .allow_origin(
            "http://localhost:5173"
                .parse::<axum::http::HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any);

    tracing_subscriber::fmt::init();

    stencil::db::init_database()
        .await
        .expect("Failed to initialize db");

    println!("Loading problems to registry...");
    stencil::load_problem_data()
        .await
        .expect("Failed to load problems");
    println!("Problems loaded!");
    println!("Loading prefixes to registry...");
    stencil::load_prefix_data()
        .await
        .expect("Failed to load prefixes");
    println!("Prefixes loaded!");

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/pdf", get(pdf_generation::send_pdf))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("\nListening on port {port}...");

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    String::from("Hello, world!")
}
