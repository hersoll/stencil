use anyhow::Result;
use db;
use registry;
use tracing::{error, info};

use crate::github::{fetch_github_releases, store_github_releases};

pub async fn initialize() -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database...");
    db::init_database().await.map_err(|e| {
        error!("Failed to initialize database: {:?}", e);
        e
    })?;
    info!("Database initialized!");

    info!("Loading problems to registry...");
    registry::load_problem_data().await.map_err(|e| {
        error!("Failed to load problems: {:?}", e);
        e
    })?;
    info!("Problems loaded!");

    info!("Loading prefixes to registry...");
    registry::load_prefix_data().await.map_err(|e| {
        error!("Failed to load prefixes: {:?}", e);
        e
    })?;
    info!("Prefixes loaded!");

    info!("Fetching GitHub releases...");
    let releases = fetch_github_releases().await?;
    store_github_releases(releases)?;
    info!("Fetched and stored GitHub releases!");

    Ok(())
}
