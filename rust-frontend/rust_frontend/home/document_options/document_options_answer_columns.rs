use dioxus::prelude::*;

use crate::frontend::i18n_lookup;
use crate::shared::{DocumentOptions, PaperSize};

#[component]
pub fn DocumentOptionsAnswerColumns(options: Signal<DocumentOptions>) -> Element {
    let min_columns = 1;
    let max_columns = 5;
    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_answer_columns\")?}:" }
            input {
                class: "input number",
                r#type: "number",
                value: "{options().answer_columns}",
                min: min_columns,
                max: max_columns,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val >= min_columns && val <= max_columns {
                            options.write().answer_columns = val;
                        }
                    }
                },
            }
        }
    }
}
