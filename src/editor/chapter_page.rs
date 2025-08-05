use crate::editor::displays::ChapterDisplay;
use crate::editor::editors::ChapterEditor;
use crate::{api, editor::displays::TopicDisplay};
use dioxus::prelude::*;

use crate::shared::{ChapterData, TopicData};

#[component]
pub fn ChapterPage() -> Element {
    let mut chapter_future = use_server_future(move || api::load_all_chapter_data())?;
    let topic_future = use_server_future(move || api::load_all_topic_ids())?;
    let mut active_chapter: Signal<Option<ChapterData>> = use_signal(|| None);
    let mut selected_chapter: Signal<Option<ChapterData>> = use_signal(|| None);
    let mut selected_topic: Signal<Option<TopicData>> = use_signal(|| None);
    let mut current_message: Signal<Option<String>> = use_signal(|| None);
    let mut chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());

    let mut used_topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());
    let mut unused_topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());

    use_effect(move || {
        if active_chapter().is_none() {
            selected_chapter.set(None);
            selected_topic.set(None);
            used_topics.set(Vec::new());
            unused_topics.set(Vec::new());
        }
    });

    use_effect(move || match chapter_future().unwrap() {
        Ok(chapter_vec) => chapters.set(chapter_vec),
        Err(message) => current_message.set(Some(message.to_string())),
    });

    let _ = use_resource(move || async move {
        if let Some(chapter) = active_chapter() {
            match crate::api::load_chapter_topics(chapter.id).await {
                Ok(chapter_topics) => match topic_future().unwrap() {
                    Ok(topic_ids) => {
                        let used_ids: Vec<i32> = chapter_topics.iter().map(|ch| ch.id).collect();
                        let mut unused_ids = topic_ids.clone();
                        unused_ids.retain(|id| !used_ids.contains(id));
                        match api::load_topics_by_id(unused_ids).await {
                            Ok(topics) => unused_topics.set(topics),
                            Err(e) => current_message.set(Some(e.to_string())),
                        }
                        used_topics.set(chapter_topics);
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
                    } else if active_chapter().is_some() {
                        TopicDisplay {
                            selected_topic,
                            topics: unused_topics,
                            current_message,
                        }
                    } else {
                        ChapterDisplay {
                            selected_chapter,
                            chapters,
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
                                chapter_future.restart();
                                current_message.set(None);
                            },
                            "OK"
                        }
                    } else if active_chapter().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(topic) = selected_topic() {
                                    if let Some(index) = used_topics().iter().position(|ch| ch == &topic) {
                                        used_topics.write().remove(index);
                                        unused_topics.write().push(topic.clone());
                                    } else if let Some(index) = unused_topics()
                                        .iter()
                                        .position(|ch| ch == &topic)
                                    {
                                        unused_topics.write().remove(index);
                                        used_topics.write().push(topic.clone());
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
                                if let Some(topic) = selected_topic()
                                    && let Some(index) = used_topics().iter().position(|ch| ch == &topic)
                                {
                                    used_topics.write().remove(index);
                                    used_topics.write().insert(index - 1, topic.clone());
                                }
                            },
                            "Up"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(topic) = selected_topic()
                                    && let Some(index) = used_topics().iter().position(|ch| ch == &topic)
                                {
                                    used_topics.write().remove(index);
                                    used_topics.write().insert(index + 1, topic.clone());
                                }
                            },
                            "Down"
                        }
                    } else {
                        button {
                            class: "button",
                            onclick: move |_| {
                                active_chapter
                                    .set(
                                        Some(ChapterData {
                                            id: 0,
                                            name: String::new(),
                                            desc_sv: String::new(),
                                            desc_en: String::new(),
                                        }),
                                    );
                                selected_chapter.set(None);
                            },
                            "Create new"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(chapter) = selected_chapter() {
                                    active_chapter.set(Some(chapter))
                                }
                            },
                            "Edit"
                        }
                        button {
                            class: "button",
                            onclick: move |_| async move {
                                if let Some(chapter) = selected_chapter() {
                                    match api::delete_chapter(chapter.id).await {
                                        Ok(deleted) => {
                                            current_message
                                                .set(Some(format!("Deleted chapter: \n {:#?}", deleted)))
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
            ChapterEditor {
                active_chapter,
                current_message,
                used_topics,
                selected_topic,
            }
        }
    }
}
