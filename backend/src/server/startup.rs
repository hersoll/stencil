use crate::{db, registry};
use tracing::{error, info};

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

    Ok(())
}
