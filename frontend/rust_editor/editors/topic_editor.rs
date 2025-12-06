use dioxus::prelude::*;

use crate::{
    api,
    editor::displays::ProblemDisplay,
    shared::{ProblemData, TopicData},
};

#[component]
pub fn TopicEditor(
    active_topic: Signal<Option<TopicData>>,
    selected_problem: Signal<Option<ProblemData>>,
    used_problems: Signal<Vec<ProblemData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "pane attributes",
            h2 { "Attributes" }
            label {
                "Name"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_topic().is_some() { false } else { true },
                    onchange: move |event| {
                        active_topic
                            .with_mut(|topic_opt| {
                                if let Some(topic) = topic_opt {
                                    topic.name = event.value();
                                }
                            });
                    },
                    value: if let Some(topic) = active_topic() { topic.name },
                }
            }
            label {
                "Description (sv)"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_topic().is_some() { false } else { true },
                    onchange: move |event| {
                        active_topic
                            .with_mut(|topic_opt| {
                                if let Some(topic) = topic_opt {
                                    topic.desc_sv = event.value();
                                }
                            });
                    },
                    value: if let Some(topic) = active_topic() { topic.desc_sv },
                }
            }
            label {
                "Description (en)"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_topic().is_some() { false } else { true },
                    onchange: move |event| {
                        active_topic
                            .with_mut(|topic_opt| {
                                if let Some(topic) = topic_opt {
                                    topic.desc_en = event.value();
                                }
                            });
                    },
                    value: if let Some(topic) = active_topic() { topic.desc_en },
                }
            }

            label {
                "Problems"
                div { class: "available_display", style: "height: 400px;",
                    ProblemDisplay {
                        problems: used_problems,
                        selected_problem,
                        current_message,
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
                        if let Some(topic) = active_topic() {
                            match api::set_topic(topic.clone()).await {
                                Ok(saved) => {
                                    match api::set_topic_problems(topic.id, used_problems().clone())
                                        .await
                                    {
                                        Ok(_) => {
                                            current_message
                                                .set(Some(format!("Saved topic:\n {:#?}", saved)))
                                        }
                                        Err(message) => current_message.set(Some(message.to_string())),
                                    }
                                }
                                Err(message) => current_message.set(Some(message.to_string())),
                            }
                        }
                        active_topic.set(None);
                    },
                    "Save"
                }
                button {
                    class: "button",
                    style: "width: 8rem;",
                    onclick: move |_| {
                        active_topic.set(None);
                    },
                    "Undo"
                }
            }
        }
    }
}
