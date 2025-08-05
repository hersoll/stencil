use dioxus::prelude::*;

use crate::shared::TopicData;

#[component]
pub fn TopicDisplay(
    selected_topic: Signal<Option<TopicData>>,
    topics: Signal<Vec<TopicData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "available_element topic header",
            style: "position: relative;",
            p { "Namn" }
            p { "Svenska" }
            p { "Engelska" }
        }
        for topic in topics() {
            div {
                class: "available_element topic item",
                style: if let Some(selected) = selected_topic() { if selected.id == topic.id { "background-color: gray;" } else { "" } },
                onclick: move |_| selected_topic.set(Some(topic.clone())),
                p { "{topic.name}" }
                p { "{topic.desc_sv}" }
                p { "{topic.desc_en}" }
            }
        }
    }
}
