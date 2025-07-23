use crate::shared::{ProblemInfo, ProblemSetData, TopicInfo};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection(topics: Signal<Vec<TopicInfo>>, set_data: Signal<ProblemSetData>) -> Element {
    let problems: Signal<Vec<ProblemInfo>> = use_signal(|| Vec::new());
    rsx! {
        for topic in topics() {
            Topic { topic, problems, set_data }
        }
    }
}

#[component]
pub fn Topic(
    topic: TopicInfo,
    problems: Signal<Vec<ProblemInfo>>,
    set_data: Signal<ProblemSetData>,
) -> Element {
    let selected = set_data().topics.contains(&topic);
    let class = if selected { "topic selected" } else { "topic" };
    // Clone the values you need before moving into closure
    let topic_id = topic.id;
    let topic_desc = topic.desc.clone();
    let topic_for_closure = topic.clone();

    rsx! {
        button {
            key: "{topic_id}",
            class,
            onclick: move |_| {
                if selected {
                    set_data.write().topics.retain(|other_topic| other_topic.id != topic_id);
                } else {
                    set_data.write().topics.push(topic_for_closure.clone());
                }
            },
            "{topic_desc}"
        }
    }
}
