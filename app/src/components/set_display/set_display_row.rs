use crate::components::set_display::set_display_editing_panel::SetDisplayEditingPanel;
use crate::enum_to_str;
use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::{APP_LANGUAGE, Error, backend::Difficulty, frontend_types::Sets, str_to_enum};

#[component]
pub fn SetDisplayRow(sets: Signal<Sets>, index: usize, max_descriptions: usize) -> Element {
    let mut editing = use_signal(|| false);
    let difficulties = vec![
        Difficulty::Intro,
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
    ];
    let mut set = sets()[index];
    rsx! {
        div { class: "set_row",
            p { class: "set_row_topics",
                if set().ids.len() > max_descriptions {
                    {
                        let first_set = set().ids.first().unwrap().clone();
                        let first_set_desc = first_set
                            .desc
                            .get(APP_LANGUAGE())
                            .ok_or(Error::NoSuchKeyExists {
                                key: APP_LANGUAGE().to_string(),
                            })?;
                        rsx! {
                        "{first_set_desc} + {set().ids.len() - 1}"
                        }
                    }
                } else {
                    {
                        let names: Result<Vec<String>, Error> = set
                            .read()
                            .ids
                            .iter()
                            .map(|id| {
                                id.desc
                                    .get(APP_LANGUAGE())
                                    .ok_or(Error::NoSuchKeyExists {
                                        key: APP_LANGUAGE().to_string(),
                                    })
                                    .map(|s| s.clone())
                            })
                            .collect();
                        let names = names?;
                        rsx! {
                        "{names.join(\", \")}"
                        }
                    }
                }
            }
            div { class: "set_row_difficulties",
                select {
                    class: "set_row_starting",
                    value: "{enum_to_str(&set().starting_difficulty)}",
                    onchange: move |opt| {
                        let chosen_difficulty = str_to_enum(&opt.value());
                        set.write().starting_difficulty = chosen_difficulty;
                        if set.write().ending_difficulty < chosen_difficulty {
                            set.write().ending_difficulty = chosen_difficulty;
                        }
                    },
                    for difficulty in difficulties.iter() {
                        option {
                            value: "{enum_to_str(&difficulty)}",
                            selected: difficulty == &set().starting_difficulty,
                            "{i18n_lookup(enum_to_str(&difficulty))?}"
                        }
                    }
                }
                "{char::from_u32(0x2192).unwrap()}"
                select {
                    class: "set_row_ending",
                    value: "{enum_to_str(&set().ending_difficulty)}",
                    onchange: move |opt| {
                        let chosen_difficulty = str_to_enum(&opt.value());
                        set.write().ending_difficulty = chosen_difficulty;
                    },
                    for difficulty in difficulties.iter() {
                        option {
                            selected: difficulty == &set().ending_difficulty,
                            value: "{enum_to_str(&difficulty)}",
                            disabled: difficulty < &set().starting_difficulty,
                            "{i18n_lookup(enum_to_str(&difficulty))?}"
                        }
                    }
                }
            }
            div { class: "set_row_number",

                input {
                    r#type: "number",
                    value: "{set().n}",
                    min: 1,
                    max: 200,
                    oninput: move |evt| {
                        if let Ok(val) = evt.value().parse::<u8>() {
                            if val > 0 && val <= 200 {
                                set.write().n = val;
                            }
                        }
                    },
                }
            }
            div { class: "set_row_controls",
                p {
                    class: "set_row_editing",
                    onclick: move |_| {
                        editing.set(!editing());
                    },
                    "{char::from_u32(0x270E).unwrap()}"
                }
                p {
                    class: "set_row_removal",
                    onclick: move |_| {
                        sets.write().remove(index);
                    },
                    "{char::from_u32(0x2573).unwrap()}"
                }
            }
        }
        SetDisplayEditingPanel {
            key: "{set().key}",
            sets,
            index,
            editing,
        }
    }
}
