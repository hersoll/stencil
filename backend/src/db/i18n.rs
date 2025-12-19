use std::collections::HashMap;

use crate::{db, Language};
use anyhow::{Context, Result};

pub async fn get_i18n_for_web(lang: &Language) -> Result<HashMap<String, String>> {
    let pool = db::get_pool();
    let data = sqlx::query!(
        "SELECT key, CASE 
                            WHEN $1 = 'sv' THEN sv
                            WHEN $1 = 'en' THEN en
                            ELSE sv
                        END as value
            FROM i18n_web",
        lang.to_str()
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

pub async fn get(key: &str, lang: &Language) -> Result<String> {
    let pool = db::get_pool();
    let data = sqlx::query!(
        "SELECT CASE 
                        WHEN $1 = 'sv' THEN sv
                        WHEN $1 = 'en' THEN en
                        ELSE sv
                    END as value
            FROM i18n_pdf
            WHERE key = $2",
        lang.to_str(),
        key
    )
    .fetch_one(pool)
    .await
    .context("failed to load translations")?;
    Ok(data.value.unwrap_or_default())
}

pub async fn get_multiple(keys: Vec<&str>, lang: &Language) -> Result<HashMap<String, String>> {
    let pool = db::get_pool();
    let owned_keys: Vec<String> = keys.into_iter().map(|s| s.to_string()).collect();
    let data = sqlx::query!(
        "SELECT key, CASE 
                            WHEN $1 = 'sv' THEN sv
                            WHEN $1 = 'en' THEN en
                            ELSE sv
                        END as value
            FROM i18n_pdf
            WHERE key = ANY($2)",
        lang.to_str(),
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
