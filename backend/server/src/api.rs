use types::{errors::ApiError, lang::Language};

pub mod editing;
pub mod public;
pub mod stats;
pub mod testing;

pub(crate) fn parse_language(lang: &str) -> Result<Language, ApiError> {
    match lang {
        "sv" => Ok(Language::Sv),
        "en" => Ok(Language::En),
        _ => Err(ApiError::BadRequest("Invalid language".to_string())),
    }
}
