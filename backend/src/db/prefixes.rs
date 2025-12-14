use super::common::{error_context, error_context_by_name};
use crate::db::{self, PrefixEntry, PrefixTranslations};
use anyhow::{Context, Result};

struct DbPrefixRow {
    id: i32,
    name: String,
    text_sv: String,
    text_en: String,
    group_text_sv: String,
    group_text_en: String,
}

impl From<DbPrefixRow> for PrefixEntry {
    fn from(row: DbPrefixRow) -> Self {
        PrefixEntry {
            id: row.id,
            name: row.name,
            translations: PrefixTranslations {
                sv: crate::db::TranslatedPrefix {
                    text: row.text_sv,
                    group_text: row.group_text_sv,
                },
                en: crate::db::TranslatedPrefix {
                    text: row.text_en,
                    group_text: row.group_text_en,
                },
            },
        }
    }
}

/// Get all prefixes
pub async fn get_all_prefix_data() -> Result<Vec<PrefixEntry>> {
    let pool = db::get_pool();
    let prefixes = sqlx::query_as!(
        DbPrefixRow,
        r#"SELECT id, name, text_sv, text_en, group_text_sv, group_text_en FROM prefixes"#
    )
    .fetch_all(pool)
    .await?;

    Ok(prefixes.into_iter().map(PrefixEntry::from).collect())
}

/// Create a new prefix
pub async fn create_prefix(prefix: PrefixEntry) -> Result<i32> {
    let pool = db::get_pool();
    let translations = prefix.translations;
    let result = sqlx::query!(
        r#"INSERT INTO prefixes (name, text_sv, text_en, group_text_sv, group_text_en) 
            VALUES ($1, $2, $3, $4, $5) 
            RETURNING id"#,
        prefix.name,
        translations.sv.text,
        translations.en.text,
        translations.sv.group_text,
        translations.en.group_text
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "prefix", &prefix.name))?;

    Ok(result.id)
}

/// Update an existing prefix
pub async fn update_prefix(prefix: PrefixEntry) -> Result<i32> {
    let pool = db::get_pool();
    let translations = prefix.translations;
    let result = sqlx::query!(
            r#"UPDATE prefixes SET name = $2, text_sv = $3, text_en = $4, group_text_sv = $5, group_text_en = $6 
            WHERE id = $1
            RETURNING id"#,
            prefix.id,
            prefix.name,
            translations.sv.text,
            translations.en.text,
            translations.sv.group_text,
            translations.en.group_text
        )
        .fetch_one(pool)
        .await
        .with_context(|| error_context("update", "prefix", prefix.id))?;

    Ok(result.id)
}
