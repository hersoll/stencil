use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::{components::ToolTip, frontend_types::ProblemSetData};

#[component]
pub fn SetOptionTitle(set: Signal<ProblemSetData>) -> Element {
    rsx! {
        div { class: "set_option_heading",
            p { "{i18n_lookup(\"set_option_title\")?}:" }
            textarea {
                class: "textarea",
                placeholder: "{i18n_lookup(\"set_option_title_placeholder\")?}",
                value: "{set().options.title}",
                onchange: move |evt| { set.write().options.title = evt.value() },
            }
            ToolTip { content: "{i18n_lookup(\"tooltip_set_title\")?}" }
        }
    }
}
