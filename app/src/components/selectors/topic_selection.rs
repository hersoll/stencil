use crate::{backend::*, APP_LANGUAGE};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection(topics: Signal<Vec<TopicData>>) -> Element {
    let translations = use_context::<Translations>();
    let mut selected_topic_name = use_signal(|| Option::<String>::None);
    let mut problems: Signal<Vec<ProblemData>> = use_signal(|| Vec::new());
    use_effect(move || {
        if let Some(topic_name) = selected_topic_name() {
            if let Some(topic) = topics().iter().find(|topic| topic.name == topic_name) {
                problems.set(topic.problems.clone());
            } else {
                throw_error(crate::Error::NoTopicWithTopicName { name: topic_name });
            }
        } else {
            problems.set(Vec::new())
        }
    });
    let selection_default = translations.get_phrase("topic_selector", APP_LANGUAGE())?;
    rsx! {
        // Topics
        if topics().len() > 0 {
            select {
                onchange: move |ev| {
                    selected_topic_name.set(Some(ev.value().to_string()));
                },
                option {
                    value: "",
                    selected: selected_topic_name().is_none(),
                    disabled: true,
                    "{selection_default}"
                }
                {
                    topics
                        .iter()
                        .map(|topic| {
                            let topic_desc = topic.get_desc(APP_LANGUAGE())?;
                            rsx! {
                                option { value: topic.name.clone(), "{topic_desc}" }
                            }
                        })
                }
            }
            // Problems
            if problems().len() > 0 {
                ul {
                    {
                        problems
                            .iter()
                            .map(|problem| {
                                let problem_desc = problem.get_desc(APP_LANGUAGE())?;
                                rsx! {
                                    li { "{problem_desc}" }
                                }
                            })
                    }
                }
            }
        }
    }
}
