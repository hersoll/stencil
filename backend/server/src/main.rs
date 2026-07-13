pub mod api;
pub mod config;
pub mod middleware;
pub mod pdf_generation;
pub mod router;
pub mod startup;

use std::net::SocketAddr;

use axum::Router;
use tracing::info;

use tracing::error;
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
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    run().await.unwrap_or_else(|_| error!("Server run failed!"));
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting server...");
    let config = config::ServerConfig::from_env();
    startup::initialize().await?;
    let router = router::create_router();
    serve(&config.addr(), router).await?;
    Ok(())
}

async fn serve(addr: &str, router: Router) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!();
    info!("Listening on {}...", listener.local_addr()?);

    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}
