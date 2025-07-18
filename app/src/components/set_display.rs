use crate::APP_LANGUAGE;
use crate::Error;
use crate::enum_to_str;
use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::frontend_types::Sets;

#[component]
pub fn SetDisplay(sets: Signal<Sets>) -> Element {
    // Descriptions to show in the topic display (first column) before changing to +N
    let max_desciptions = 3;

    let mut editing_rows: Signal<Vec<usize>> = use_signal(|| Vec::new());

    rsx! {
        div { class: "set_display_legend",
                p { class: "set_row_topics", "{i18n_lookup(\"topics\")?}:" }
                p { "{i18n_lookup(\"difficulty\")?}:" }
                p { "{i18n_lookup(\"number_of_problems\")?}:" }
                p { "" }
            }
        div { class: "set_display",

            if sets().len() == 0 {
                div { class: "set_row_placeholder",
                    p { "{i18n_lookup(\"please_create_a_set\")?}" }
                    p {}
                    p {}
                    p {}
                }
            }
            for (i , set) in sets().iter().enumerate() {
                div { class: "set_row", key: "set_row_{i}",
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
                    p { class:"set_row_controls",
                        div { class: "set_row_editing",
                        onclick: move |_| {
                            if editing_rows().contains(&i) {
                                editing_rows.write().retain(|row| *row != i);
                            } else {
                                editing_rows.write().push(i);
                            }
                        },
                        "{char::from_u32(0x270E).unwrap()}" }
                        div {
                            class: "set_row_removal",
                            onclick: move |_| {
                                sets.write().remove(i);
                            },
                            "{char::from_u32(0x2573).unwrap()}"
                        }
                    }
                }
                div {
                    class: if editing_rows().contains(&i) {"editing_display editing"} else {"editing_display"},
                    "lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd lorem dsds ds dsd s ds d s da d sa \n\n ds ds ds ds ds d sd s ds ds d sd "
                }
            }
        }
    }
}
