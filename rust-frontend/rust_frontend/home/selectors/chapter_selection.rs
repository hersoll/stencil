use crate::api::load_chapter_descs;
use crate::frontend::APP_LANGUAGE;
use dioxus::prelude::*;

#[component]
pub fn ChapterSelection(
    chapters: Signal<Vec<i32>>,
    active_chapter: Signal<i32>,
    topics: Signal<Vec<i32>>,
) -> Element {
    let chapter_descs = use_resource(move || async move {
        if let Ok(descs) = load_chapter_descs(chapters(), APP_LANGUAGE()).await {
            descs
        } else {
            Vec::new()
        }
    });
    rsx! {
        if let Some(descs) = chapter_descs() && descs.len() == chapters().len() {
            for (i , desc) in descs.iter().enumerate() {
                {
                    let id = chapters()[i];
                    let selected_class = if active_chapter() == id { "selected" } else { "" };
                    rsx! {
                        button {
                            key: "{id}",
                            class: "chapter {selected_class}",
                            onclick: move |_| async move {
                                active_chapter.set(id);
                                match crate::api::load_chapter_topic_ids(id).await {
                                    Ok(api_topics) => topics.set(api_topics),
                                    Err(_) => topics.set(Vec::new()),
                                }
                            },
                            "{desc}"
                        }
                    }
                }
            }
        }
    }
}
