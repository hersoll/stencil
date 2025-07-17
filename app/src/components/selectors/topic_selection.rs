use crate::{APP_LANGUAGE, TRANSLATIONS, backend::*, frontend_types::ProblemSetData};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection(topics: Signal<Vec<TopicData>>, set_data: Signal<ProblemSetData>) -> Element {
    let problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    rsx! {
        for topic in topics() {
            Topic { topic, problems, set_data }
        }
    }
}

#[component]
pub fn Topic(
    topic: TopicData,
    problems: Signal<Vec<ProblemData>>,
    set_data: Signal<ProblemSetData>,
) -> Element {
    let selected = set_data().ids.contains(&topic);
    let topic_desc = &topic.get_desc(APP_LANGUAGE())?;
    let class = if selected { "topic selected" } else { "topic" };
    rsx! {
        button {
            key: "{topic.name.clone()}",
            class,
            onclick: move |_| {
                problems.set(topic.problems.clone());
                if selected {
                    set_data.write().ids.retain(|id| id.name != topic.name);
                } else {
                    set_data.write().ids.push(topic.clone());
                }
            },
            "{topic_desc}"
        }
    }
}
