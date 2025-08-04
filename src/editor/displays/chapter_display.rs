use dioxus::prelude::*;

use crate::shared::ChapterData;

#[component]
pub fn ChapterDisplay(
    selected_chapter: Signal<Option<ChapterData>>,
    chapters: Signal<Vec<ChapterData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "available_element chapter header",
            style: "position: relative;",
            p { "Namn" }
            p { "Svenska" }
            p { "Engelska" }
        }
        for chapter in chapters() {
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
