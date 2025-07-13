use dioxus::prelude::*;
use crate::APP_LANGUAGE;

#[component]
pub fn LanguageSwitch() -> Element {
    rsx! {
        button {
            onclick: move |_| {
                if APP_LANGUAGE == "sv" {
                    *APP_LANGUAGE.write() = "en";
                } else {
                    *APP_LANGUAGE.write() = "sv";
                }
            },
            if APP_LANGUAGE == "sv" {
                "English"
            } else {
                "Svenska"
            }
        }
    }
}
