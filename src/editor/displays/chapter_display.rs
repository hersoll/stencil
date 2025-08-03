use dioxus::prelude::*;

use crate::{editor::displays::update_arrow::UpdateArrow, shared::ChapterData};

#[component]
pub fn ChapterDisplay(
    selected_chapter: Signal<Option<ChapterData>>,
    chapter_future: Resource<Result<Vec<ChapterData>, ServerFnError>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "available_element chapter header",
            style: "position: relative;",
            p { "Namn" }
            p { "Svenska" }
            p { "Engelska" }
            UpdateArrow { future: chapter_future }
        }
        match chapter_future().unwrap() {
            Ok(chapters) => {
                rsx! {
                    for chapter in chapters {
                        div {
                            class: "available_element chapter item",
                            style: if let Some(selected) = selected_chapter() { if selected.id == chapter.id { "background-color: gray;" } else { "" } },
                            onclick: move |_| selected_chapter.set(Some(chapter.clone())),
                            p { "{chapter.name}" }
                            p { "{chapter.desc_sv}" }
                            p { "{chapter.desc_en}" }
                        }
                    }
                }
            }
            Err(message) => {
                current_message.set(Some(message.to_string()));
                rsx! {}
            }
        }
    }
}
