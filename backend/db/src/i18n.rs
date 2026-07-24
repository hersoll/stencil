use anyhow::{Context, Result};
use std::collections::HashMap;
use types::lang::Language;

pub async fn get_i18n_for_web(lang: &Language) -> Result<HashMap<String, String>> {
    let pool = crate::get_pool();
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

pub async fn get_pdf_translations(lang: &Language) -> Result<HashMap<String, String>> {
    let pool = crate::get_pool();
    let data = sqlx::query!(
        "SELECT key, CASE 
                            WHEN $1 = 'sv' THEN sv
                            WHEN $1 = 'en' THEN en
                            ELSE sv
                        END as value
            FROM i18n_pdf;",
        lang.to_str(),
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
