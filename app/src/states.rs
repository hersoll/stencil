use dioxus::prelude::*;
const DEFAULT_LANGUAGE: &str = "sv";
pub static APP_LANGUAGE: GlobalSignal<&str> = Global::new(|| DEFAULT_LANGUAGE);
