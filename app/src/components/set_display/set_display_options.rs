use dioxus::prelude::*;

use crate::frontend_types::Sets;

#[component]
pub fn SetDisplayOptions(sets: Signal<Sets>, index: usize) -> Element {
    let mut set = sets()[index];
    rsx! {
        div { class: "set_options",
            input {
                class: "question_columns",
                r#type: "number",
                value: "{set().question_columns}",
                min: 1,
                max: 5,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val > 0 && val <= 6 {
                            set.write().question_columns = val;
                        }
                    }
                },
            }
        }
    }
}
