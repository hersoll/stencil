use std::collections::HashMap;

use crate::db;
use anyhow::{Context, Result};

pub struct I18nDatabase;

impl I18nDatabase {
    pub async fn get_i18n(lang: &str) -> Result<HashMap<String, String>> {
        let pool = db::get_pool();
        let data = sqlx::query!(
            "SELECT key, CASE 
                            WHEN $1 = 'sv' THEN sv
                            WHEN $1 = 'en' THEN en
                            ELSE sv
                        END as value
            FROM i18n",
            lang
        )
        .fetch_all(pool)
        .await
        .context("failed to load translations")?;
        let map = data
            .into_iter()
            .map(|row| (row.key, row.value.unwrap_or_default()))
            .collect();

        Ok(map)
    }

    pub async fn get(key: &str, lang: &str) -> Result<String> {
        let pool = db::get_pool();
        let data = sqlx::query!(
            "SELECT CASE 
                        WHEN $1 = 'sv' THEN sv
                        WHEN $1 = 'en' THEN en
                        ELSE sv
                    END as value
            FROM i18n
            WHERE key = $2",
            lang,
            key
        )
        .fetch_one(pool)
        .await
        .context("failed to load translations")?;
        Ok(data.value.unwrap_or_default())
    }

    pub async fn get_multiple(keys: Vec<&str>, lang: &str) -> Result<HashMap<String, String>> {
        let pool = db::get_pool();
        let owned_keys: Vec<String> = keys.into_iter().map(|s| s.to_string()).collect();
        let data = sqlx::query!(
            "SELECT key, CASE 
                            WHEN $1 = 'sv' THEN sv
                            WHEN $1 = 'en' THEN en
                            ELSE sv
                        END as value
            FROM i18n
            WHERE key = ANY($2)",
            lang,
            &owned_keys
        )
        .fetch_all(pool)
        .await
        .context("failed to load translations")?;
        let map = data
            .into_iter()
            .map(|row| (row.key, row.value.unwrap_or_default()))
            .collect();

        Ok(map)
    }
}
