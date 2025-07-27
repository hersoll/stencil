use crate::frontend::{i18n_lookup, Sets};
use crate::shared::Difficulty;
use dioxus::prelude::*;

use crate::frontend::APP_LANGUAGE;

#[component]
pub fn ProblemExclusions(sets: Signal<Sets>, index: usize) -> Element {
    let mut set = sets()[index];
    let api_data = use_memo(move || {
        (
            set().topics,
            set().starting_difficulty,
            set().ending_difficulty,
        )
    });
    let fetch_result = use_resource(move || {
        let (topics, starting_difficulty, ending_difficulty) = api_data();
        async move {
            crate::api::load_valid_problems(
                topics,
                starting_difficulty,
                ending_difficulty,
                APP_LANGUAGE(),
            )
            .await
        }
    });

    rsx! {
        div { class: "problem_exclusions",
            if let Some(Ok(problems)) = fetch_result() {
                for problem in problems {
                    {
                        let excluded = set().exclusions.contains(&problem.id);
                        rsx! {
                            div {
                                class: if excluded { "exclusion_row excluded" } else { "exclusion_row" },
                                onclick: move |_| {
                                    if excluded {
                                        set.write().exclusions.retain(|&id| id != problem.id);
                                    } else {
                                        set.write().exclusions.push(problem.id);
                                    }
                                },
                                p { class: "exclusion_desc", "{problem.desc}" }
                                p { class: "exclusion_difficulty",
                                    "{i18n_lookup(&Difficulty::num_to_enum(problem.difficulty.try_into().unwrap())?.to_str())?}"
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
