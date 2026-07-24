use crate::{ID, Name, PublicFlag, error_context, error_context_by_name};
use types::lang::Language;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

struct DbPrefixRow {
    id: ID,
    name: Name,
    text_sv: String,
    text_en: String,
    group_text_sv: String,
    group_text_en: String,
    public: PublicFlag,
}

/// The texts associated with a specific prefix in a certain [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixTexts {
    pub text: String,
    pub group_text: String,
}

/// Contains [`PrefixTexts`] for every [`Language`]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixTranslations {
    pub sv: PrefixTexts,
    pub en: PrefixTexts,
}

/// Representation of prefix data in the DB
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixEntry {
    pub id: ID,
    pub name: Name,
    pub translations: PrefixTranslations,
}
/// The same data as [`ProblemEntry`], except it includes information about whether the entry is
/// public or not.
///
/// This is needed so the editor can edit the `public` value
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PrefixEntryForEditor {
    #[serde(flatten)]
    pub entry: PrefixEntry,
    pub public: PublicFlag,
}

impl PrefixEntry {
    /// Get the text in a specific [`Language`] for a prefix in its singular form.
    pub fn get_text(&self, lang: Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.text,
            Language::En => &self.translations.en.text,
        }
    }

    /// Get the text in a specific [`Language`] for a prefix in its group form.
    pub fn get_group_text(&self, lang: Language) -> &str {
        match lang {
            Language::Sv => &self.translations.sv.group_text,
            Language::En => &self.translations.en.group_text,
        }
    }
}

impl From<DbPrefixRow> for PrefixEntry {
    fn from(row: DbPrefixRow) -> Self {
        PrefixEntry {
            id: row.id,
            name: row.name,
            translations: PrefixTranslations {
                sv: PrefixTexts {
                    text: row.text_sv,
                    group_text: row.group_text_sv,
                },
                en: PrefixTexts {
                    text: row.text_en,
                    group_text: row.group_text_en,
                },
            },
        }
    }
}

impl From<DbPrefixRow> for PrefixEntryForEditor {
    fn from(row: DbPrefixRow) -> Self {
        PrefixEntryForEditor {
            public: row.public,
            entry: PrefixEntry::from(row),
        }
    }
}

/// Retrive data about every prefix in the DB
///
/// Used by the editor to list all prefixes
pub async fn get_all_prefix_data() -> Result<Vec<PrefixEntryForEditor>> {
    let pool = crate::get_pool();
    let prefixes = sqlx::query_as!(
        DbPrefixRow,
        r#"SELECT id, name, text_sv, text_en, group_text_sv, group_text_en, public
        FROM prefixes"#
    )
    .fetch_all(pool)
    .await?;

    Ok(prefixes
        .into_iter()
        .map(PrefixEntryForEditor::from)
        .collect())
}

/// Retrive data about every *public* prefix in the DB
///
/// Used during startup by the `registry` module
pub async fn get_public_prefix_data() -> Result<Vec<PrefixEntry>> {
    // In prod we only want the public rows,
    // in dev we want all
    let production_mode = cfg!(feature = "docker") || std::env::args().any(|x| x == "prod");

    let pool = crate::get_pool();
    let prefixes = sqlx::query_as!(
        DbPrefixRow,
        r#"SELECT id, name, text_sv, text_en, group_text_sv, group_text_en, public
        FROM prefixes
        WHERE (NOT $1::bool OR public)"#,
        production_mode
    )
    .fetch_all(pool)
    .await?;

    Ok(prefixes.into_iter().map(PrefixEntry::from).collect())
}

/// Get a specific prefix using its ID
///
/// Used in the problem editor do display information about the connected prefix
pub async fn get_prefix_from_id(id: &i32) -> Result<PrefixEntryForEditor> {
    let pool = crate::get_pool();
    let prefix_row = sqlx::query_as!(
        DbPrefixRow,
        r#"SELECT id, name, text_sv, text_en, group_text_sv, group_text_en, public
        FROM prefixes WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await?;

    Ok(PrefixEntryForEditor::from(prefix_row))
}

pub async fn create_prefix_from_entry(prefix: PrefixEntryForEditor) -> Result<i32> {
    let pool = crate::get_pool();
    let translations = prefix.entry.translations;
    let result = sqlx::query!(
        r#"INSERT INTO prefixes (name, text_sv, text_en, group_text_sv, group_text_en, public) 
            VALUES ($1, $2, $3, $4, $5, $6) 
            RETURNING id"#,
        prefix.entry.name,
        translations.sv.text,
        translations.en.text,
        translations.sv.group_text,
        translations.en.group_text,
        prefix.public
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context_by_name("create", "prefix", &prefix.entry.name))?;

    Ok(result.id)
}

pub async fn update_prefix_from_entry(prefix: PrefixEntryForEditor) -> Result<String> {
    let pool = crate::get_pool();
    let translations = prefix.entry.translations;
    let result = sqlx::query!(
        r#"UPDATE prefixes SET name = $2, text_sv = $3, text_en = $4, group_text_sv = $5,
            group_text_en = $6, public = $7
            WHERE id = $1
            RETURNING name"#,
        prefix.entry.id,
        prefix.entry.name,
        translations.sv.text,
        translations.en.text,
        translations.sv.group_text,
        translations.en.group_text,
        prefix.public
    )
    .fetch_one(pool)
    .await
    .with_context(|| error_context("update", "prefix", prefix.entry.id))?;

    Ok(result.name)
}

pub async fn delete_prefix_with_id(id: i32) -> Result<String> {
    let pool = crate::get_pool();
    let result = sqlx::query!(r#"DELETE FROM prefixes WHERE id = $1 RETURNING name"#, id)
        .fetch_one(pool)
        .await
        .with_context(|| error_context("delete", "prefix", id))?;

    Ok(result.name)
}
