use dioxus::prelude::*;

use crate::{APP_LANGUAGE, TRANSLATIONS, backend::Difficulty, frontend_types::ProblemSetData};
use crate::{enum_to_str, i18n_lookup, str_to_enum};

#[component]
pub fn DifficultyPicker(set_data: Signal<ProblemSetData>) -> Element {
    let from_str = TRANSLATIONS().get_phrase("from", APP_LANGUAGE())?;
    let to_str = TRANSLATIONS().get_phrase("to", APP_LANGUAGE())?;
    let difficulty_str = TRANSLATIONS().get_phrase("difficulty", APP_LANGUAGE())?;

    let difficulties = vec![
        Difficulty::Intro,
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
    ];

    rsx! {
        div { class: "difficulty_picker",
            h3 { "{difficulty_str}:" }
            p { "{from_str}" }
            select {
                onchange: move |opt| {
                    let mut set = set_data.write();
                    let chosen_difficulty = str_to_enum(&opt.value());
                    set.starting_difficulty = chosen_difficulty;
                    if set.ending_difficulty < chosen_difficulty {
                        set.ending_difficulty = chosen_difficulty;
                    }
                },
                for difficulty in difficulties.iter() {
                    option { value: "{enum_to_str(&difficulty)}",
                        "{i18n_lookup(enum_to_str(&difficulty))?}"
                    }
                }
            }
            p { "{to_str}" }
            select {
                value: "{enum_to_str(&set_data().ending_difficulty)}",
                onchange: move |opt| {
                    let mut set = set_data.write();
                    let chosen_difficulty = str_to_enum(&opt.value());
                    set.ending_difficulty = chosen_difficulty;
                },
                for difficulty in difficulties {
                    option {
                        value: "{enum_to_str(&difficulty)}",
                        disabled: difficulty < set_data().starting_difficulty,
                        "{i18n_lookup(enum_to_str(&difficulty))?}"
                    }
                }
            }
        }
    }
}
