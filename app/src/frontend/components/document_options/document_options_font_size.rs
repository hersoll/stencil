use dioxus::prelude::*;

use crate::frontend::i18n_lookup;
use crate::shared::DocumentOptions;

#[component]
pub fn DocumentOptionsFontSize(options: Signal<DocumentOptions>) -> Element {
    let min_size = 6;
    let max_size = 24;
    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_font_size\")?}:" }
            input {
                class: "input number",
                r#type: "number",
                value: "{options().font_size}",
                min: min_size,
                max: max_size,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val >= min_size && val <= max_size {
                            options.write().font_size = val;
                        }
                    }
                },
            }
        }
    }
}
