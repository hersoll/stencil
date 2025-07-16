use crate::enum_to_str;
use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::frontend_types::{ProblemSetData, Sets};

#[component]
pub fn AddSet(set_data: Signal<ProblemSetData>, sets: Signal<Sets>) -> Element {
    rsx! {
        button {
            class: "add_set_button",
            onclick: move |_| sets.write().push(set_data()),
            "Add Set"
        }

        for set in sets() {
            div { class: "set_display",
                p {
                    if set.ids.len() > 1 {
                        "{set.ids.first().unwrap()} + {set.ids.len() - 1}"
                    } else {
                        "{set.ids.first().unwrap()}"
                    }
                }
                p { "{i18n_lookup(enum_to_str(&set.starting_difficulty))?}" }
                p { "{i18n_lookup(enum_to_str(&set.ending_difficulty))?}" }
                p { "{set.n}" }
            }
        }
    }
}
