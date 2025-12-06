use dioxus::prelude::*;

use crate::frontend::{ToolTip, i18n_lookup};
use crate::shared::DocumentOptions;

#[component]
pub fn DocumentOptionsHeading(options: Signal<DocumentOptions>) -> Element {
    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_heading\")?}:" }
            input {
                class: "input text",
                oninput: move |evt| {
                    options.write().heading = evt.value();
                },
            }
            ToolTip { content: "{i18n_lookup(\"tooltip_document_heading\")?}" }
        }
    }
}
