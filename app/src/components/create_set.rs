use crate::enum_to_str;
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

        div { class: "set_display",
            div { class: "set_display_legend",
                p { class: "set_row_topics", "{i18n_lookup(\"topics\")?}:" }
                p { "{i18n_lookup(\"difficulty\")?}:" }
                p { "{i18n_lookup(\"number_of_problems\")?}:" }
                p { "" }
            }
            for (i , set) in sets().iter().enumerate() {
                div { class: "set_row",
                    p { class: "set_row_topics",
                        if set.ids.len() > 2 {
                            "{set.ids.first().unwrap()} + {set.ids.len() - 1}"
                        } else {
                            "{set.ids.join(\", \")}"
                        }
                    }
                    p {
                        {
                            if set.starting_difficulty == set.ending_difficulty {
                                rsx! {
                                "{i18n_lookup(enum_to_str(&set.starting_difficulty))?}"
                                }
                            } else {
                                rsx! {
                                "{i18n_lookup(enum_to_str(&set.starting_difficulty))?} {char::from_u32(0x2192).unwrap()} {i18n_lookup(enum_to_str(&set.ending_difficulty))?}"
                                }
                            }
                        }
                    }
                    p { "{set.n}" }
                    p {
                        class: "set_row_removal",
                        onclick: move |_| {
                            sets.write().remove(i);
                        },
                        "{char::from_u32(0x2573).unwrap()}"
                    }
                }
            }
        }
    }
}
