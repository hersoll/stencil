use dioxus::prelude::*;

use crate::components::LanguageSwitch;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { id: "header",

            h1 { "Ekvata" }
            LanguageSwitch {}
        }
    }
}
