use crate::{APP_LANGUAGE, TRANSLATIONS, backend::*};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection(topics: Signal<Vec<TopicData>>, active_topic: Signal<String>) -> Element {
    let mut problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    rsx! {
        for topic in topics() {
            Topic { topic, active_topic, problems }
        }
    }
}

#[component]
pub fn Topic(
    topic: TopicData,
    active_topic: Signal<String>,
    problems: Signal<Vec<ProblemData>>,
) -> Element {
    let mut selected = use_signal(|| false);
    let topic_desc = &topic.get_desc(APP_LANGUAGE())?;
    let class = if selected() {
        "topic selected"
    } else {
        "topic"
    };
    rsx! {
        button {
            key: "{topic.name.clone()}",
            class,
            onclick: move |_| {
                selected.set(!selected());
                active_topic.set(topic.name.clone());
                problems.set(topic.problems.clone());
            },
            "{topic_desc}"
        }
    }
}
