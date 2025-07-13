use std::collections::HashMap;

use dioxus::prelude::*;

use crate::backend::Translations;

// LANGUAGE
const DEFAULT_LANGUAGE: &str = "sv";
pub static APP_LANGUAGE: GlobalSignal<&str> = Global::new(|| DEFAULT_LANGUAGE);

// TRANSLATIONS
pub static TRANSLATIONS: GlobalSignal<Translations> = Global::new(|| Translations::new(HashMap::new()));

