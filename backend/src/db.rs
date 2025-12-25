pub mod api;
mod chapters;
mod common;
mod courses;
pub mod i18n;
mod prefixes;
mod problems;
mod relationships;
mod topics;
mod types;
pub mod users;

pub use chapters::*;
pub use courses::*;
pub use prefixes::*;
pub use problems::*;
pub use topics::*;
pub use types::*;

use anyhow::{Context, Result, anyhow};
use once_cell::sync::OnceCell;
use sqlx::{PgPool, postgres::PgPoolOptions};

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_database() -> Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(1)
        .connect(&database_url)
        .await
        .context("Unable to initialize DB Pool options")?;

    // Store the pool globally
    DB_POOL
        .set(pool)
        .map_err(|_| anyhow!("Failed to set database pool"))?;

    // Test the connection
    let pool = get_pool();
    let _ = sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .context("Test connection to db failed")?;

    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    let pool = DB_POOL.get().expect("Database pool not initialized");

    pool
}
