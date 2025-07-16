use dioxus::prelude::*;

use crate::frontend_types::ProblemSetData;

#[component]
pub fn NumberPicker(set_data: Signal<ProblemSetData>) -> Element {
    rsx! {
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
