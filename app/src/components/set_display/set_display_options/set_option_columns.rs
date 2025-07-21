use dioxus::prelude::*;

use crate::{frontend_types::ProblemSetData, i18n_lookup};

#[component]
pub fn SetOptionColumns(set: Signal<ProblemSetData>) -> Element {
    let min_columns = 1;
    let max_columns = 5;
    rsx! {
        div {
            p { "{i18n_lookup(\"set_option_columns\")?}:" }
            input {
                class: "input",
                r#type: "number",
                value: "{set().options.question_columns}",
                min: min_columns,
                max: max_columns,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val >= min_columns && val <= max_columns {
                            set.write().options.question_columns = val;
                        }
                    }
                },
            }
        }
    }
}
