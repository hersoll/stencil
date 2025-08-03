mod i18n;
mod problems;

pub use i18n::I18nDatabase;
pub use problems::ProblemDatabase;

use once_cell::sync::OnceCell;
use sqlx::{postgres::PgPoolOptions, PgPool};

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_database() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
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

    // Test the connection
    sqlx::query("SELECT 1").execute(&pool).await?;
    println!("Database connected successfully!");

    // Store the pool globally
    DB_POOL
        .set(pool)
        .map_err(|_| "Failed to set database pool")?;

    Ok(())
}

pub fn get_pool() -> &'static PgPool {
    DB_POOL.get().expect("Database pool not initialized")
}
