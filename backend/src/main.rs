use tracing_subscriber;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

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

    let app = Router::new().route("/", get(hello_world));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> String {
    String::from("Hello, world!")
}
