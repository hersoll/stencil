use crate::editor::displays::ProblemDisplay;
use crate::editor::editors::ProblemEditor;
use crate::{api, editor::displays::PrefixDisplay};
use dioxus::prelude::*;

use crate::shared::{PrefixData, ProblemData};

#[component]
pub fn ProblemPage() -> Element {
    let mut problem_future = use_server_future(move || api::load_all_problem_data())?;
    let mut prefix_future = use_server_future(move || api::load_all_prefix_data())?;
    let mut active_problem: Signal<Option<ProblemData>> = use_signal(|| None);
    let mut selected_problem: Signal<Option<ProblemData>> = use_signal(|| None);
    let mut selected_prefix: Signal<Option<PrefixData>> = use_signal(|| None);
    let mut current_message: Signal<Option<String>> = use_signal(|| None);
    let mut problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    let mut prefixes: Signal<Vec<PrefixData>> = use_signal(|| Vec::new());

    let mut used_prefix: Signal<PrefixData> = use_signal(|| PrefixData::new());

    use_effect(move || {
        if active_problem().is_none() {
            selected_problem.set(None);
            selected_prefix.set(None);
            used_prefix.set(PrefixData::new());
        }
    });

    use_effect(move || match problem_future().unwrap() {
        Ok(problem_vec) => {
            problems.set(problem_vec);
        }
        Err(message) => current_message.set(Some(message.to_string())),
    });

    use_effect(move || match prefix_future().unwrap() {
        Ok(prefix_vec) => prefixes.set(prefix_vec),
        Err(message) => current_message.set(Some(message.to_string())),
    });

    use_effect(move || {
        if let Some(problem) = active_problem()
            && let Some(prefix_id) = problem.prefix_id
        {
            used_prefix.set(match prefixes.iter().find(|pf| pf.id == prefix_id) {
                Some(prefix) => prefix.clone(),
                None => PrefixData::new(),
            })
        } else {
            used_prefix.set(PrefixData::new())
        }
    });

    rsx! {
        div { class: "editor_container",
            div { class: "pane available",
                div { class: "available_display", style: "height: 760px;",
                    if let Some(message) = current_message() {
                        div { style: "padding: 1rem; font-weight: 200;", "{message}" }
                    } else if active_problem().is_some() {
                        PrefixDisplay { selected_prefix, prefixes }
                    } else {
                        ProblemDisplay {
                            selected_problem,
                            problems,
                            current_message,
                        }
                    }
                }
                div {
                    class: "button_container",
                    style: "display: flex; gap: 1rem;",
                    if current_message().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                problem_future.restart();
                                prefix_future.restart();
                                current_message.set(None);
                            },
                            "OK"
                        }
                    } else if active_problem().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                used_prefix.set(PrefixData::new());
                            },
                            "Create new"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                used_prefix
                                    .set(
                                        match selected_prefix() {
                                            Some(prefix) => prefix,
                                            None => PrefixData::new(),
                                        },
                                    );
                            },
                            "Use"
                        }
                    } else {
                        button {
                            class: "button",
                            onclick: move |_| {
                                active_problem
                                    .set(
                                        Some(ProblemData {
                                            id: 0,
                                            difficulty: 0,
                                            module: String::new(),
                                            name: String::new(),
                                            desc_sv: String::new(),
                                            desc_en: String::new(),
                                            question_sv: None,
                                            question_en: None,
                                            answer_sv: None,
                                            answer_en: None,
                                            solution_sv: None,
                                            solution_en: None,
                                            prefix_id: None,
                                        }),
                                    );
                                selected_problem.set(None);
                            },
                            "Create new"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(problem) = selected_problem() {
                                    active_problem.set(Some(problem))
                                }
                            },
                            "Edit"
                        }
                        button {
                            class: "button",
                            onclick: move |_| async move {
                                if let Some(problem) = selected_problem() {
                                    match api::delete_problem(problem.id).await {
                                        Ok(deleted) => {
                                            current_message
                                                .set(Some(format!("Deleted problem: \n {:#?}", deleted)))
                                        }
                                        Err(message) => current_message.set(Some(message.to_string())),
                                    }
                                }
                            },
                            "Delete"
                        }
                    }
                }
            }
            ProblemEditor { active_problem, current_message, used_prefix }
        }
    }
}
