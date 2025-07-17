use crate::APP_LANGUAGE;
use crate::Error;
use crate::enum_to_str;
use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::frontend_types::Sets;

#[component]
pub fn SetDisplay(sets: Signal<Sets>) -> Element {
    // Descriptions to show in the topic display (first column)
    let max_desciptions = 3;
    rsx! {
        div { class: "set_display",
            div { class: "set_display_legend",
                p { class: "set_row_topics", "{i18n_lookup(\"topics\")?}:" }
                p { "{i18n_lookup(\"difficulty\")?}:" }
                p { "{i18n_lookup(\"number_of_problems\")?}:" }
                p { "" }
            }
            if sets().len() == 0 {
                div { class: "set_row_placeholder",
                    p { "{i18n_lookup(\"please_create_a_set\")?}" }
                    p {}
                    p {}
                    p {}
                }
            }
            for (i , set) in sets().iter().enumerate() {
                div { class: "set_row",
                    p { class: "set_row_topics",
                        if set.ids.len() > max_desciptions {
                            {
                                let first_set = set.ids.first().unwrap();
                                let first_set_desc = first_set
                                    .desc
                                    .get(APP_LANGUAGE())
                                    .ok_or(Error::NoSuchKeyExists {
                                        key: APP_LANGUAGE().to_string(),
                                    })?;
                                rsx! {
                                "{first_set_desc} + {set.ids.len() - 1}"
                                }
                            }
                        } else {
                            {
                                let mut names: Vec<String> = Vec::new();
                                let mut id_iter = set.ids.iter();
                                for _ in 0..set.ids.len() {
                                    names
                                        .push(
                                            id_iter
                                                .next()
                                                .unwrap()
                                                .desc
                                                .get(APP_LANGUAGE())
                                                .ok_or(Error::NoSuchKeyExists {
                                                    key: APP_LANGUAGE().to_string(),
                                                })?
                                                .clone(),
                                        );
                                }
                                rsx! {
                                "{names.join(\", \")}"
                                }
                            }
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
