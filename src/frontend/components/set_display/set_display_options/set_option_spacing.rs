use dioxus::prelude::*;

use crate::frontend::{components::ToolTip, i18n_lookup};
use crate::shared::ProblemSetData;

#[component]
pub fn SetOptionSpacing(set: Signal<ProblemSetData>) -> Element {
    let min_spacing = 0;
    let max_spacing = 250;
    rsx! {
        div {
            p { "{i18n_lookup(\"set_option_spacing\")?}:" }
            input {
                class: "input number",
                r#type: "number",
                min: min_spacing,
                max: max_spacing,
                value: if let Some(spacing) = set().options.spacing { "{spacing}" },
                onchange: move |evt| {
                    if let Ok(val) = evt.value().parse::<u16>() {
                        if val >= min_spacing && val <= max_spacing {
                            set.write().options.spacing = Some(val);
                        }
                    } else if evt.value().is_empty() {
                        set.write().options.spacing = None;
                    }
                },
            }
            p { "mm" }
            ToolTip { content: i18n_lookup("tooltip_set_spacing")? }
        }
    }
}
