use dioxus::prelude::*;

use crate::frontend::home::LanguageSwitch;

#[component]
pub fn Header() -> Element {
    rsx! {
        div { id: "header",

            h1 { "Stencil" }
            LanguageSwitch {}
        }
    }
}
