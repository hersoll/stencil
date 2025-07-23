use dioxus::prelude::*;

use crate::frontend::i18n_lookup;
use crate::shared::DocumentOptions;

#[component]
pub fn DocumentOptionsParSpacing(options: Signal<DocumentOptions>) -> Element {
    let min_spacing = 0;
    let max_spacing = 100;
    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_spacing\")?}:" }
            input {
                class: "input number",
                r#type: "number",
                value: if let Some(spacing) = options().par_spacing {
                    "{spacing}"
        },
                min: min_spacing,
                max: max_spacing,
                oninput: move |evt| {
                    if let Ok(val) = evt.value().parse::<u8>() {
                        if val >= min_spacing && val <= max_spacing {
                            options.write().par_spacing = Some(val);
                        }
                    }   else if evt.value().is_empty() {
                        options.write().par_spacing = None;
                    }
                },
            }
            p {"mm"}
        }
    }
}
