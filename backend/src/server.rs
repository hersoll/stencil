pub mod config;
pub mod middleware;
pub mod router;
pub mod startup;

use std::net::SocketAddr;

use axum::Router;
use tracing::info;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
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
