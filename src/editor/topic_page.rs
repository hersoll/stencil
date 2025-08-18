use crate::editor::displays::TopicDisplay;
use crate::editor::editors::TopicEditor;
use crate::{api, editor::displays::ProblemDisplay};
use dioxus::prelude::*;

use crate::shared::{ProblemData, TopicData};

#[component]
pub fn TopicPage() -> Element {
    let mut topic_future = use_server_future(move || api::load_all_topic_data())?;
    let problem_future = use_server_future(move || api::load_all_problem_ids())?;
    let mut active_topic: Signal<Option<TopicData>> = use_signal(|| None);
    let mut selected_topic: Signal<Option<TopicData>> = use_signal(|| None);
    let mut selected_problem: Signal<Option<ProblemData>> = use_signal(|| None);
    let mut current_message: Signal<Option<String>> = use_signal(|| None);
    let mut topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());

    let mut used_problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    let mut unused_problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());

    use_effect(move || {
        if active_topic().is_none() {
            selected_topic.set(None);
            selected_problem.set(None);
            used_problems.set(Vec::new());
            unused_problems.set(Vec::new());
        }
    });

    use_effect(move || match topic_future().unwrap() {
        Ok(topic_vec) => topics.set(topic_vec),
        Err(message) => current_message.set(Some(message.to_string())),
    });

    let _ = use_resource(move || async move {
        if let Some(topic) = active_topic() {
            match crate::api::load_topic_problems(topic.id).await {
                Ok(topic_problems) => match problem_future().unwrap() {
                    Ok(problem_ids) => {
                        let used_ids: Vec<i32> = topic_problems.iter().map(|ch| ch.id).collect();
                        let mut unused_ids = problem_ids.clone();
                        unused_ids.retain(|id| !used_ids.contains(id));
                        match api::load_problems_by_id(unused_ids).await {
                            Ok(problems) => unused_problems.set(problems),
                            Err(e) => current_message.set(Some(e.to_string())),
                        }
                        used_problems.set(topic_problems);
                    }
                    Err(e) => current_message.set(Some(e.to_string())),
                },
                Err(e) => current_message.set(Some(e.to_string())),
            }
        }
    });

    rsx! {
        div { class: "editor_container",
            div { class: "pane available",
                div { class: "available_display", style: "height: 760px;",
                    if let Some(message) = current_message() {
                        div { style: "padding: 1rem; font-weight: 200;", "{message}" }
                    } else if active_topic().is_some() {
                        ProblemDisplay {
                            selected_problem,
                            problems: unused_problems,
                            current_message,
                        }
                    } else {
                        TopicDisplay {
                            selected_topic,
                            topics,
                            current_message,
                        }
                    }
                }
                div {
                    class: "button_container",
                    style: "display: flex; flex-wrap: wrap; gap: 1rem;",
                    if current_message().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                topic_future.restart();
                                current_message.set(None);
                            },
                            "OK"
                        }
                    } else if active_topic().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(problem) = selected_problem() {
                                    if let Some(index) = used_problems().iter().position(|ch| ch == &problem) {
                                        used_problems.write().remove(index);
                                        unused_problems.write().push(problem.clone());
                                    } else if let Some(index) = unused_problems()
                                        .iter()
                                        .position(|ch| ch == &problem)
                                    {
                                        unused_problems.write().remove(index);
                                        used_problems.write().push(problem.clone());
                                    } else {
                                        current_message
                                            .set(Some(String::from("This button desn't fucking work!")))
                                    }
                                }
                            },
                            "Move"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(problem) = selected_problem()
                                    && let Some(index) = used_problems().iter().position(|ch| ch == &problem)
                                {
                                    used_problems.write().remove(index);
                                    used_problems.write().insert(index - 1, problem.clone());
                                }
                            },
                            "Up"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(problem) = selected_problem()
                                    && let Some(index) = used_problems().iter().position(|ch| ch == &problem)
                                {
                                    used_problems.write().remove(index);
                                    used_problems.write().insert(index + 1, problem.clone());
                                }
                            },
                            "Down"
                        }
                    } else {
                        button {
                            class: "button",
                            onclick: move |_| {
                                active_topic
                                    .set(
                                        Some(TopicData {
                                            id: 0,
                                            name: String::new(),
                                            desc_sv: String::new(),
                                            desc_en: String::new(),
                                        }),
                                    );
                                selected_topic.set(None);
                            },
                            "Create new"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(topic) = selected_topic() {
                                    active_topic.set(Some(topic))
                                }
                            },
                            "Edit"
                        }
                        button {
                            class: "button",
                            onclick: move |_| async move {
                                if let Some(topic) = selected_topic() {
                                    match api::delete_topic(topic.id).await {
                                        Ok(deleted) => {
                                            current_message
                                                .set(Some(format!("Deleted topic: \n {:#?}", deleted)))
                                        }
                                        Err(message) => current_message.set(Some(message.to_string())),
                                    }
                                }
                            },
                            "Delete"
                        }
                        button {
                            class: "button",
                            onclick: move |_| async move {
                                if let Some(topic) = selected_topic() {
                                    let mut new_topic = topic.clone();
                                    new_topic.id = 0;
                                    active_topic.set(Some(new_topic));
                                }
                            },
                            "Copy"
                        }
                    }
                }
            }
            TopicEditor {
                active_topic,
                current_message,
                used_problems,
                selected_problem,
            }
        }
    }
}
