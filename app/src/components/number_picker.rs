use dioxus::prelude::*;

use crate::{frontend_types::ProblemSetData, i18n_lookup};

#[component]
pub fn NumberPicker(set_data: Signal<ProblemSetData>) -> Element {
    let pick_a_number_str = i18n_lookup("pick_number")?;
    rsx! {
        div { class: "number_picker",

            p { "{pick_a_number_str}:" }

            input {
                r#type: "number",
                min: 1,
                max: 200,
                value: "{set_data().n}",
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val > 0 && val <= 200 {
                            set_data.write().n = val;
                        }
                    }
                },
            }
        }
    }
}
