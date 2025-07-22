use dioxus::prelude::*;

use crate::shared::{DocumentOptions, PaperSize};
use crate::frontend::i18n_lookup;

#[component]
pub fn DocumentOptionsPaperSize(options: Signal<DocumentOptions>) -> Element {
    let paper_sizes = vec![
        PaperSize::A4,
        PaperSize::A5,
    ];
    let mut value = use_signal(|| PaperSize::A4);
    use_effect(move || {
        value.set(options().paper_size);
    });
    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_paper_size\")?}:" }
            select {
                value: "{value().to_typst()}",
                class: "select with_arrow",
                onchange: move |evt| {
                    options.write().paper_size = PaperSize::from(&evt.value());
                },
                for size in paper_sizes {
                    option {value: "{size.to_typst()}", "{size.to_str()}"}
                }
            }
        }
    }
}
