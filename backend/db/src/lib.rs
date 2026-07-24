pub mod chapters;
pub mod courses;
pub mod i18n;
pub mod logging;
pub mod prefixes;
pub mod problems;
pub mod relationships;
pub mod topics;
pub mod users;

use anyhow::{Context, Result, anyhow};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};

use types::lang::Language;

/// Used in functions that access data that can be public or private (field in the db)
///
/// Normally, the program reads the feature flags to automatically determine whether to read
/// private data. However, in the editor we always want private data as well, so we want to tell
/// the program to ignore feature flags and just read all of the data.
pub struct ForceReadPrivateData(pub bool);

/// Contains a description for every [`Language`]
///
/// Descriptions are used on multiple structs. They explain to the user what that
/// specific problem, topic, etc. is.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptionTranslations {
    pub sv: String,
    pub en: String,
}

/// Generic database row for entities with id, name, and descriptions
pub(crate) struct DbDescRow {
    pub id: i32,
    pub name: String,
    pub desc_sv: String,
    pub desc_en: String,
}

impl DbDescRow {
    /// Convert to DescriptionTranslations
    pub fn into_desc_translations(self) -> (i32, String, DescriptionTranslations) {
        (
            self.id,
            self.name,
            DescriptionTranslations {
                sv: self.desc_sv,
                en: self.desc_en,
            },
        )
    }
}

/// Trait for all structs with a [`DescriptionTranslations`] field.
///
/// Used to easily access the description in the [`Language`] you want.
pub trait HasDesc {
    /// Accesses the [`DescriptionTranslations`] field.
    ///
    /// Required for `get_desc_for_lang()`, not intended for external use.
    fn desc(&self) -> &DescriptionTranslations;

    /// Get the description in the specified [`Language`].
    fn get_desc_for_lang(&self, lang: Language) -> String {
        match lang {
            Language::Sv => self.desc().sv.clone(),
            Language::En => self.desc().en.clone(),
        }
    }
}

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
    DB_POOL.get().expect("Database pool not initialized")
}

/// Function that reduces boilerplate when writing DB errors
pub(crate) fn error_context(
    action: &str,
    entity: &str,
    identifier: impl std::fmt::Display,
) -> String {
    format!("Failed to {} {} with id {}", action, entity, identifier)
}

/// Function that reduces boilerplate when writing DB errors
pub(crate) fn error_context_by_name(action: &str, entity: &str, name: &str) -> String {
    format!("Failed to {} {} '{}'", action, entity, name)
}
