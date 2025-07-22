use crate::shared::ProblemSetData;
use dioxus::prelude::*;

use crate::frontend::components::ToolTip;
use crate::frontend::i18n_lookup;
use crate::shared::Difficulty;

#[component]
pub fn DifficultyPicker(set_data: Signal<ProblemSetData>) -> Element {
    let to_str = i18n_lookup("to")?;
    let difficulty_str = i18n_lookup("difficulty")?;
    let difficulties = vec![
        Difficulty::Intro,
        Difficulty::Easy,
        Difficulty::Medium,
        Difficulty::Hard,
    ];

    rsx! {
        div { class: "difficulty_picker",
            p { "{difficulty_str}:" }
            select {
                class: "select with_arrow",
                onchange: move |opt| {
                    let mut set = set_data.write();
                    let chosen_difficulty = Difficulty::str_to_enum(&opt.value());
                    set.starting_difficulty = chosen_difficulty;
                    if set.ending_difficulty < chosen_difficulty {
                        set.ending_difficulty = chosen_difficulty;
                    }
                },
                for difficulty in difficulties.iter() {
                    option { value: "{difficulty.to_str()}", "{i18n_lookup(difficulty.to_str())?}" }
                }
            }
            p { "{to_str}" }
            select {
                class: "select with_arrow",
                value: "{set_data().ending_difficulty.to_str()}",
                onchange: move |opt| {
                    let mut set = set_data.write();
                    let chosen_difficulty = Difficulty::str_to_enum(&opt.value());
                    set.ending_difficulty = chosen_difficulty;
                },
                for difficulty in difficulties {
                    option {
                        value: "{difficulty.to_str()}",
                        disabled: difficulty < set_data().starting_difficulty,
                        "{i18n_lookup(difficulty.to_str())?}"
                    }
                }
            }
            ToolTip { content: "<p><strong>Intro:</strong> Mjukstartsuppgifter</p><p><strong>Lätt:</strong> Ungefär E-nivå</p><p><strong>Medel:</strong> Ungefär C-nivå</p><p><strong>Svår:</strong> Upp till A-nivå</p><p>Notera att uppgifter i början av kursen är lättare än senare avsnitt och överensstämmer inte nödvändigtvis med betygsstegen lika väl.</p>" }
        }
    }
}
