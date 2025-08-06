use dioxus::prelude::*;

use crate::{
    api,
    shared::{PrefixData, ProblemData},
};

#[component]
pub fn ProblemEditor(
    active_problem: Signal<Option<ProblemData>>,
    used_prefix: Signal<PrefixData>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "pane problem_editor", style: "font-size: 0.8rem;",
            h2 { "Attributes" }
            div { style: "display: flex; gap: 1rem;",
                label {
                    "Module"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 12rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        problem.module = event.value();
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { problem.module },
                    }
                }
                label {
                    "Name"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 16rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        problem.name = event.value();
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { problem.name },
                    }
                }
                label {
                    "Difficulty"
                    input {
                        class: "input number",
                        style: "font-size: 0.6rem; width: 5rem;",
                        r#type: "number",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        problem.difficulty = match event.value().parse() {
                                            Ok(val) => val,
                                            Err(e) => {
                                                current_message.set(Some(e.to_string()));
                                                0
                                            }
                                        };
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { problem.difficulty },
                    }
                }
            }
            div { style: "display: flex; gap: 1rem;",

                label {
                    "Description (sv)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        problem.desc_sv = event.value();
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { problem.desc_sv },
                    }
                }
                label {
                    "Description (en)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        problem.desc_en = event.value();
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { problem.desc_en },
                    }
                }
            }
            div { style: "display: flex; gap: 1rem;",
                label {
                    "Question (sv)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        if event.value().is_empty() {
                                            problem.question_sv = None;
                                        } else {
                                            problem.question_sv = Some(event.value());
                                        }
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { match problem.question_sv {
                            Some(s) => s,
                            None => String::new(),
                        } },
                    }
                }
                label {
                    "Question (en)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        if event.value().is_empty() {
                                            problem.question_en = None;
                                        } else {
                                            problem.question_en = Some(event.value());
                                        }
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { match problem.question_en {
                            Some(s) => s,
                            None => String::new(),
                        } },
                    }
                }
            }
            div { style: "display: flex; gap: 1rem;",
                label {
                    "Answer (sv)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        if event.value().is_empty() {
                                            problem.answer_sv = None;
                                        } else {
                                            problem.answer_sv = Some(event.value());
                                        }
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { match problem.answer_sv {
                            Some(s) => s,
                            None => String::new(),
                        } },
                    }
                }
                label {
                    "Answer (en)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        if event.value().is_empty() {
                                            problem.answer_en = None;
                                        } else {
                                            problem.answer_en = Some(event.value());
                                        }
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { match problem.answer_en {
                            Some(s) => s,
                            None => String::new(),
                        } },
                    }
                }
            }

            div { style: "display: flex; gap: 1rem;",
                label {
                    "Solution (sv)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        if event.value().is_empty() {
                                            problem.solution_sv = None;
                                        } else {
                                            problem.solution_sv = Some(event.value());
                                        }
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { match problem.solution_sv {
                            Some(s) => s,
                            None => String::new(),
                        } },
                    }
                }
                label {
                    "Solution (en)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            active_problem
                                .with_mut(|problem_opt| {
                                    if let Some(problem) = problem_opt {
                                        if event.value().is_empty() {
                                            problem.solution_en = None;
                                        } else {
                                            problem.solution_en = Some(event.value());
                                        }
                                    }
                                });
                        },
                        value: if let Some(problem) = active_problem() { match problem.solution_en {
                            Some(s) => s,
                            None => String::new(),
                        } },
                    }
                }
            }

            h2 { "Prefix" }
            label {
                "Name"
                input {
                    class: "input text",
                    style: "font-size: 0.6rem; width: 17rem;",
                    r#type: "text",
                    disabled: if active_problem().is_some() { false } else { true },
                    onchange: move |event| {
                        used_prefix.write().name = event.value();
                    },
                    value: used_prefix().name,
                }
            }
            div { style: "display: flex; gap: 1rem;",
                label {
                    "Text (sv)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            used_prefix.write().text_sv = event.value();
                        },
                        value: used_prefix().text_sv,
                    }
                }
                label {
                    "Text (en)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            used_prefix.write().text_en = event.value();
                        },
                        value: used_prefix().text_en,
                    }
                }
            }

            div { style: "display: flex; gap: 1rem;",
                label {
                    "Group text (sv)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            used_prefix.write().group_text_sv = event.value();
                        },
                        value: used_prefix().group_text_sv,
                    }
                }
                label {
                    "Group text (en)"
                    input {
                        class: "input text",
                        style: "font-size: 0.6rem; width: 17rem;",
                        r#type: "text",
                        disabled: if active_problem().is_some() { false } else { true },
                        onchange: move |event| {
                            used_prefix.write().group_text_en = event.value();
                        },
                        value: used_prefix().group_text_en,
                    }
                }
            }




            div {
                class: "button_container",
                style: "display: flex; gap: 1rem; justify-content: center;",
                button {
                    class: "button",
                    style: "width: 15rem;",
                    onclick: move |_| async move {
                        if used_prefix().name.is_empty() {
                            active_problem.as_mut().unwrap().prefix_id = None;
                        } else {
                            match api::set_prefix(used_prefix().clone()).await {
                                Ok(id) => active_problem.as_mut().unwrap().prefix_id = Some(id),
                                Err(e) => current_message.set(Some(e.to_string())),
                            }
                        }
                        if let Some(problem) = active_problem() {
                            match api::set_problem(problem.clone()).await {
                                Ok(saved) => {
                                    current_message
                                        .set(Some(format!("Saved problem with id:\n {:#?}", saved)))
                                }
                                Err(message) => current_message.set(Some(message.to_string())),
                            }
                        }
                        active_problem.set(None);
                    },
                    "Save"
                }
                button {
                    class: "button",
                    style: "width: 8rem;",
                    onclick: move |_| {
                        active_problem.set(None);
                    },
                    "Undo"
                }
            }
        }
    }
}
