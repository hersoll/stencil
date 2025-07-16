use crate::{APP_LANGUAGE, backend::*};
use dioxus::prelude::*;

#[component]
pub fn ChapterSelection(
    chapters: Signal<Vec<ChapterData>>,
    active_chapter: Signal<String>,
    topics: Signal<Vec<TopicData>>,
) -> Element {
    rsx! {
        for chapter in chapters() {
            {
                let selected_class = if active_chapter() == chapter.name {
                    "selected"
                } else {
                    ""
                };
                let chapter_desc = &chapter.get_desc(APP_LANGUAGE())?;
                rsx! {
                    button {
                        key: "{chapter.name.clone()}",
                        class: "chapter {selected_class}",
                        onclick: move |_| {
                            active_chapter.set(chapter.name.clone());
                            topics.set(chapter.topics.clone());
                        },
                        "{chapter_desc}"
                    }
                }
            }
        }
    }
}
