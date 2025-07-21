use dioxus::prelude::*;

use crate::frontend_types::ProblemSetData;

#[component]
pub fn SetOptionColumns(set: Signal<ProblemSetData>) -> Element {
    rsx! {
        div { class: "set_option_columns",
            p { "Kolumner:" }
            input {
                class: "input",
                style: "width: 2rem;",
                r#type: "number",
                value: "{set().options.question_columns}",
                min: 1,
                max: 5,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val > 0 && val <= 6 {
                            set.write().options.question_columns = val;
                        }
                    }
                },
            }
        }
    }
}
