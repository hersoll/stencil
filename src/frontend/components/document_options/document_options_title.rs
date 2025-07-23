use dioxus::prelude::*;

use crate::frontend::i18n_lookup;
use crate::shared::{DocumentOptions, PaperSize};

#[component]
pub fn DocumentOptionsTitle(options: Signal<DocumentOptions>) -> Element {
    rsx! {
    div {
        p { "{i18n_lookup(\"document_option_title\")?}:" }
        textarea {
            class: "textarea",
            oninput: move |evt| {
                        options.write(). = evt.value();
                }
            },
        }
    }
}
