use crate::frontend::types::TooltipData;
use dioxus::prelude::*;
use std::collections::HashMap;

// LANGUAGE
const DEFAULT_LANGUAGE: &str = "sv";
pub static APP_LANGUAGE: GlobalSignal<&str> = Global::new(|| DEFAULT_LANGUAGE);

// TRANSLATIONS
pub static TRANSLATIONS: GlobalSignal<HashMap<String, String>> =
    Global::new(|| HashMap::new());

pub static TOOLTIP: GlobalSignal<TooltipData> = Global::new(|| TooltipData {
    content: String::new(),
    visible: false,
});
