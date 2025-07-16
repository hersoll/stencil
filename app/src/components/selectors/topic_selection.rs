use crate::{APP_LANGUAGE, TRANSLATIONS, backend::*, frontend_types::ProblemSetData};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection(
    topics: Signal<Vec<TopicData>>,
    active_topic: Signal<String>,
    set_data: Signal<ProblemSetData>,
) -> Element {
    let mut problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    rsx! {
        for topic in topics() {
            Topic {
                topic,
                active_topic,
                problems,
                set_data,
            }
        }
    }
}

#[component]
pub fn Topic(
    topic: TopicData,
    active_topic: Signal<String>,
    problems: Signal<Vec<ProblemData>>,
    set_data: Signal<ProblemSetData>,
) -> Element {
    let selected = set_data().ids.contains(&topic.name);
    let topic_desc = &topic.get_desc(APP_LANGUAGE())?;
    let class = if selected { "topic selected" } else { "topic" };
    rsx! {
        button {
            key: "{topic.name.clone()}",
            class,
            onclick: move |_| {
                active_topic.set(topic.name.clone());
                problems.set(topic.problems.clone());
                if selected {
                    set_data.write().ids.retain(|id| id != &topic.name);
                } else {
                    set_data.write().ids.push(topic.name.clone());
                }
            },
            "{topic_desc}"
        }
    }
}
