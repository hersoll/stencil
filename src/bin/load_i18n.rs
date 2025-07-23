// src/bin/load_i18n.rs - Load i18n JSON into database 
use sqlx::{Pool, Postgres};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type PgPool = Pool<Postgres>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Translation {
    pub sv: String,
    pub en: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| {
            println!("DATABASE_URL not found, using default...");
            "postgresql://postgres:password@localhost:5432/courses_db".to_string()
        });
    
    println!("Connecting to: {}", database_url);
    let pool = PgPool::connect(&database_url).await?;

    // Read the JSON file
    let json_content = std::fs::read_to_string("translations.json")?;
    let translations: HashMap<String, Translation> = serde_json::from_str(&json_content)?;

    println!("Found {} translation keys", translations.len());

    // Start transaction
    let mut tx = pool.begin().await?;

    // Clear existing i18n data (optional)
    println!("Clearing existing i18n data...");
    sqlx::query("DELETE FROM i18n").execute(&mut *tx).await?;

    // Insert translations
    for (key, translation) in &translations {
        sqlx::query!(
            "INSERT INTO i18n (key, sv, en) VALUES ($1, $2, $3)",
            key,
            translation.sv,
            translation.en
        ).execute(&mut *tx).await?;
        
        println!("Inserted: {} -> sv: '{}', en: '{}'", key, translation.sv, translation.en);
    }

    // Commit transaction
    tx.commit().await?;

    println!("Successfully loaded {} translations into i18n table!", translations.len());

    // Verify the data
    let count = sqlx::query_scalar!("SELECT COUNT(*) FROM i18n")
        .fetch_one(&pool)
        .await?;
    
    println!("Verification: i18n table now has {} rows", count.unwrap_or(0));

    Ok(())
}
