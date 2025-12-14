use crate::db::DescriptionTranslations;

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

/// Standard error message formatting
pub(crate) fn error_context(
    action: &str,
    entity: &str,
    identifier: impl std::fmt::Display,
) -> String {
    format!("Failed to {} {} with id {}", action, entity, identifier)
}

pub(crate) fn error_context_by_name(action: &str, entity: &str, name: &str) -> String {
    format!("Failed to {} {} '{}'", action, entity, name)
}
