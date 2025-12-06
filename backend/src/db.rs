mod i18n;
mod problems;

pub use i18n::I18nDatabase;
pub use problems::ProblemDatabase;

use dotenvy;
use once_cell::sync::OnceCell;
use sqlx::{PgPool, postgres::PgPoolOptions};

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // Ensure UTF-8 encoding
                sqlx::query("SET client_encoding = 'UTF8'")
                    .execute(conn)
                    .await?;
                Ok(())
            })
        })
        .connect(&database_url)
        .await?;

    // Store the pool globally
    DB_POOL
        .set(pool)
        .map_err(|_| "Failed to set database pool")?;

    // Test the connection
    let pool = get_pool();
    let _ = sqlx::query("SELECT 1").execute(pool).await?;
    println!("Database connected successfully!");

    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    let pool = DB_POOL.get().expect("Database pool not initialized");

    pool
}
