use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::frontend_types::{ProblemSetData, Sets};

#[component]
pub fn CreateSet(set_data: Signal<ProblemSetData>, sets: Signal<Sets>) -> Element {
    // True if you try to add set without adding a problem type
    let mut creation_error = use_signal(|| false);
    use_effect(move || {
        if !set_data().ids.is_empty() {
            creation_error.set(false);
        }
    });
    rsx! {
        div { class: "create_set_button_container",
            button {
                class: "create_set_button",
                onclick: move |_| {
                    if set_data().ids.is_empty() {
                        creation_error.set(true);
                    } else {
                        creation_error.set(false);
                        sets.write().push(set_data());
                    }
                },
                "{i18n_lookup(\"create_set\")?}"
            }
        }

        div { class: "create_set_error",
            if creation_error() {
                "{i18n_lookup(\"create_set_error\")?}"
            } else {
                ""
            }
        }
    }
}
