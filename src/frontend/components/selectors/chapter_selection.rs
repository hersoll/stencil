use crate::frontend::APP_LANGUAGE;
use crate::shared::ChapterInfo;
use crate::shared::TopicInfo;
use dioxus::prelude::*;

#[component]
pub fn ChapterSelection(
    chapters: Signal<Vec<ChapterInfo>>,
    active_chapter: Signal<i32>,
    topics: Signal<Vec<TopicInfo>>,
) -> Element {
    rsx! {
        for chapter in chapters() {
            {
                let selected_class = if active_chapter() == chapter.id {
                    "selected"
                } else {
                    ""
                };
                rsx! {
                    button {
                        key: "{chapter.name.clone()}",
                        class: "chapter {selected_class}",
                        onclick: move |_| async move {
                            active_chapter.set(chapter.id);
                            match crate::api::load_chapter_topics(chapter.id, APP_LANGUAGE().to_string()).await {
                                Ok(topic_info) => topics.set(topic_info),
                                Err(_) => topics.set(Vec::new()),
                            }
                        },
                        "{chapter.desc}"
                    }
                }
            }
        }
    }
}
