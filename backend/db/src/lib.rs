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
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, postgres::PgPoolOptions};

use tracing::info;
use types::lang::Language;

pub type ID = i32;
pub type Name = String;
pub type Description = String;
pub type PublicFlag = bool;

/// Several functions are interested in whether the program is running in production mode or not.
///
/// In production mode, we only retrieve data with the column public = true. In dev mode we retrieve
/// all data. This makes it possible to work on new problems while the website keeps running as
/// usual.
pub static PRODUCTION_MODE: Lazy<bool> =
    Lazy::new(|| cfg!(feature = "docker") || std::env::args().any(|x| x == "prod"));
pub fn production_mode() -> bool {
    *PRODUCTION_MODE
}

/// A collection of fields shared by courses, chapters and topics. Problems also include them all
/// but has several extra fields, so [`problems`] has its own Row struct.
pub struct DatabaseRow {
    id: ID,
    name: Name,
    desc_sv: Description,
    desc_en: Description,
    public: PublicFlag,
}

impl DatabaseRow {
    /// Combine the Descriptions into a DescriptionTranslations struct
    pub fn as_desc_translations(&self) -> DescriptionTranslations {
        DescriptionTranslations {
            sv: self.desc_sv.clone(),
            en: self.desc_en.clone(),
        }
    }
}

/// Contains a description for every [`Language`]
///
/// Descriptions are used on multiple structs. They explain to the user what that
/// specific problem, topic, etc. is.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DescriptionTranslations {
    pub sv: String,
    pub en: String,
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

/// Start the database and test the connection
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

    if production_mode() {
        info!("Running in production mode!")
    } else {
        info!("Running in dev mode!")
    }

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
