use crate::frontend::APP_LANGUAGE;
use dioxus::prelude::*;

#[component]
pub fn LanguageSwitch() -> Element {
    let swedish = String::from("sv");
    let english = String::from("en");
    rsx! {
        button {
            id: "language_switch",
            onclick: move |_| {
                let mut lang = APP_LANGUAGE.write();
                *lang = if *lang == swedish { english.clone() } else { swedish.clone() };
            },
            if APP_LANGUAGE() == swedish {
                "English"
            } else {
                "Svenska"
            }
        }
    }
}
