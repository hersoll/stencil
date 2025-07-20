use crate::backend::Difficulty;
use crate::enum_to_str;
use crate::frontend_types::Sets;
use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::{
    APP_LANGUAGE,
    backend::{self},
};

#[component]
pub fn ProblemExclusions(sets: Signal<Sets>, index: usize) -> Element {
    let mut set = sets()[index];
    let api_data = use_memo(move || {
        (
            set().ids,
            set().starting_difficulty,
            set().ending_difficulty,
        )
    });
    let fetch_result = use_resource(move || {
        let (topics, starting_difficulty, ending_difficulty) = api_data();
        let topic_names: Vec<_> = topics.iter().map(|topic| topic.name.clone()).collect();
        async move {
            backend::get_problems(
                topic_names,
                starting_difficulty,
                ending_difficulty,
                APP_LANGUAGE().to_string(),
            )
            .await
        }
    });

    rsx! {
        div { class: "problem_exclusions",
            if let Some(Ok(problems)) = fetch_result() {
                for (name , desc , difficulty) in problems {
                    {
                        let excluded = set().exclusions.contains(&name);
                        rsx! {
                            div {
                                class: if excluded { "exclusion_row excluded" } else { "exclusion_row" },
                                onclick: move |_| {
                                    if excluded {
                                        set.write().exclusions.retain(|str| *str != name);
                                    } else {
                                        set.write().exclusions.push(name.clone());
                                    }
                                },
                                p { class: "exclusion_desc", "{desc}" }
                                p { class: "exclusion_difficulty",
                                    "{i18n_lookup(enum_to_str(&Difficulty::num_to_enum(difficulty)?))?}"
                                }
                            }
                        }
                    }
                }
            } else if let Some(Err(e)) = fetch_result() {
                // Handle the error case - either return empty or show error message
                div { class: "exclusion_row error",
                    p { "ERROR: {e}" }
                }
            } else {

                div { class: "exclusion_row loading",
                    p { "{i18n_lookup(\"loading\")?}..." }
                }
            }
        }
    }
}
