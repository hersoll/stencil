use crate::{api::load_topic_descs, frontend::APP_LANGUAGE, shared::ProblemSetData};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection(topics: Signal<Vec<i32>>, set_data: Signal<ProblemSetData>) -> Element {
    let topic_descs = use_resource(move || async move {
        if let Ok(descs) = load_topic_descs(topics(), APP_LANGUAGE()).await {
            descs
        } else {
            Vec::new()
        }
    });

    rsx! {
        if let Some(descs) = topic_descs() && descs.len() > 0 && topics().len() > 0 {
            for (i , desc) in descs.iter().enumerate() {
                Topic { topic: topics()[i], desc, set_data }
            }
        } else {

        }
    }
}

#[component]
pub fn Topic(topic: i32, desc: String, set_data: Signal<ProblemSetData>) -> Element {
    let selected = set_data().topics.contains(&topic);
    let class = if selected { "topic selected" } else { "topic" };

    rsx! {
        button {
            key: topic,
            class,
            onclick: move |_| {
                if selected {
                    set_data.write().topics.retain(|&id| id != topic);
                } else {
                    set_data.write().topics.push(topic);
                }
            },
            "{desc}"
        }
    }
}
