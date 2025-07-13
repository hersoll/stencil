use crate::{backend::*, frontend_types::Language};
use dioxus::prelude::*;

#[component]
pub fn LanguageSwitch() -> Element {
    let mut language = use_context::<Signal<Language>>();
    let current_language = language();
    rsx! {
        button {
            onclick: move |_| {
                if current_language == String::from("sv") {
                    language.set(String::from("en"));
                } else {
                    language.set(String::from("sv"));
                }
            },
            if current_language == String::from("sv") {
                "English"
            } else {
                "Svenska"
            }
        }
    }
}
