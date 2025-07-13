use crate::APP_LANGUAGE;
use dioxus::prelude::*;

#[component]
pub fn LanguageSwitch() -> Element {
    rsx! {
        button {
            id: "language_switch",
            onclick: move |_| {
                let mut lang = APP_LANGUAGE.write();
                *lang = if *lang == "sv" { "en" } else { "sv" };
            },
            if APP_LANGUAGE() == "sv" { "English" } else { "Svenska" }
        }
    }
}
