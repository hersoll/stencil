use dioxus::prelude::*;

use crate::{
    api,
    editor::displays::TopicDisplay,
    shared::{ChapterData, TopicData},
};

#[component]
pub fn ChapterEditor(
    active_chapter: Signal<Option<ChapterData>>,
    selected_topic: Signal<Option<TopicData>>,
    used_topics: Signal<Vec<TopicData>>,
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
                    disabled: if active_chapter().is_some() { false } else { true },
                    onchange: move |event| {
                        active_chapter
                            .with_mut(|chapter_opt| {
                                if let Some(chapter) = chapter_opt {
                                    chapter.name = event.value();
                                }
                            });
                    },
                    value: if let Some(chapter) = active_chapter() { chapter.name },
                }
            }
            label {
                "Description (sv)"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_chapter().is_some() { false } else { true },
                    onchange: move |event| {
                        active_chapter
                            .with_mut(|chapter_opt| {
                                if let Some(chapter) = chapter_opt {
                                    chapter.desc_sv = event.value();
                                }
                            });
                    },
                    value: if let Some(chapter) = active_chapter() { chapter.desc_sv },
                }
            }
            label {
                "Description (en)"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_chapter().is_some() { false } else { true },
                    onchange: move |event| {
                        active_chapter
                            .with_mut(|chapter_opt| {
                                if let Some(chapter) = chapter_opt {
                                    chapter.desc_en = event.value();
                                }
                            });
                    },
                    value: if let Some(chapter) = active_chapter() { chapter.desc_en },
                }
            }

            label {
                "Topics"
                div { class: "available_display", style: "height: 400px;",
                    TopicDisplay {
                        topics: used_topics,
                        selected_topic,
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
                        if let Some(chapter) = active_chapter() {
                            match api::set_chapter(chapter.clone()).await {
                                Ok(saved) => {
                                    match api::set_chapter_topics(chapter.id, used_topics().clone())
                                        .await
                                    {
                                        Ok(_) => {
                                            current_message
                                                .set(Some(format!("Saved chapter:\n {:#?}", saved)))
                                        }
                                        Err(message) => current_message.set(Some(message.to_string())),
                                    }
                                }
                                Err(message) => current_message.set(Some(message.to_string())),
                            }
                        }
                        active_chapter.set(None);
                    },
                    "Save"
                }
                button {
                    class: "button",
                    style: "width: 8rem;",
                    onclick: move |_| {
                        active_chapter.set(None);
                    },
                    "Undo"
                }
            }
        }
    }
}
