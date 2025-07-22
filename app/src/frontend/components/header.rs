use dioxus::prelude::*;

use crate::frontend::components::LanguageSwitch;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { id: "header",

            h1 { "Ekvata" }
            LanguageSwitch {}
        }
    }
}
