use dioxus::prelude::*;
mod document_options_color;
mod document_options_header;
mod document_options_lang;
mod document_options_paper_size;
mod document_options_solutions;

use document_options_color::DocumentOptionsColor;
use document_options_header::DocumentOptionsHeader;
use document_options_lang::DocumentOptionsLanguage;
use document_options_paper_size::DocumentOptionsPaperSize;
use document_options_solutions::DocumentOptionsWriteSolution;

use crate::shared::DocumentOptions;

#[component]
pub fn DocumentOptionDisplay(options: Signal<DocumentOptions>) -> Element {
    let active = use_signal(|| false);
    rsx! {
        div { class: "document_options_container",
            DocumentOptionsHeader { active }
            div { class: if active() { "document_options_panel active" } else { "document_options_panel" },
                div { class: "wrapper",
                    DocumentOptionsWriteSolution { options }
                    DocumentOptionsPaperSize { options }
                    DocumentOptionsLanguage { options }
                    DocumentOptionsColor { options }
                }
            }
        }
    }
}
