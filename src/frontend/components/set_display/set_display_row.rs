use dioxus::prelude::*;

use crate::api::{load_topic_desc};
use crate::frontend::components::set_display::set_display_editing_panel::SetDisplayEditingPanel;
use crate::frontend::{i18n_lookup, Sets, APP_LANGUAGE};
use crate::shared::Difficulty;

#[component]
pub fn SetDisplayRow(sets: Signal<Sets>, index: usize, max_descriptions: usize) -> Element {
    let mut editing = use_signal(|| false);
    let mut first_topic_desc = use_signal(|| String::new());
    let mut second_topic_desc = use_signal(|| String::new());
    let mut set = sets()[index];
    let _ = use_resource(move || async move {
        if set().topics.len() > 0 {
            if let Ok(desc) = load_topic_desc(set().topics[0], APP_LANGUAGE()).await {
                first_topic_desc.set(desc);
            }
        }
        if set().topics.len() > 1 {
            if let Ok(desc) = load_topic_desc(set().topics[1], APP_LANGUAGE()).await {
                second_topic_desc.set(desc);
            }
        }
    });
    let difficulties = vec![
        Difficulty::Intro,
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
    ];
    rsx! {
        div { class: "set_row",
            p { class: "set_row_topics",
                if set().topics.len() > max_descriptions {
                    {
                        rsx! {
                        "{first_topic_desc} + {set().topics.len() - 1}"
                        }
                    }
                } else if set().topics.len() == 2 {
                    "{first_topic_desc}, {second_topic_desc}"
                } else if set().topics.len() == 1 {

                    "{first_topic_desc}"
                }
            }
            div { class: "set_row_difficulties",
                select {
                    class: "select transparent rtl",
                    value: "{set().starting_difficulty.to_str()}",
                    onchange: move |opt| {
                        let chosen_difficulty = Difficulty::str_to_enum(&opt.value());
                        set.write().starting_difficulty = chosen_difficulty;
                        if set.write().ending_difficulty < chosen_difficulty {
                            set.write().ending_difficulty = chosen_difficulty;
                        }
                    },
                    for difficulty in difficulties.iter() {
                        option {
                            value: "{difficulty.to_str()}",
                            selected: difficulty == &set().starting_difficulty,
                            "{i18n_lookup(difficulty.to_str())?}"
                        }
                    }
                }
                "{char::from_u32(0x2192).unwrap()}"
                select {
                    class: "select transparent",
                    value: "{set().ending_difficulty.to_str()}",
                    onchange: move |opt| {
                        let chosen_difficulty = Difficulty::str_to_enum(&opt.value());
                        set.write().ending_difficulty = chosen_difficulty;
                    },
                    for difficulty in difficulties.iter() {
                        option {
                            selected: difficulty == &set().ending_difficulty,
                            value: "{difficulty.to_str()}",
                            disabled: difficulty < &set().starting_difficulty,
                            "{i18n_lookup(difficulty.to_str())?}"
                        }
                    }
                }
            }
            input {
                class: "input transparent number",
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
            div {
                class: "set_row_controls",
                style: "border-left: 1px gray solid; line-height: 1.2rem;",
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
