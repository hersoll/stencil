use crate::backend::{self, ProblemRegistry};
use dioxus::prelude::*;

#[component]
pub fn TopicSelection() -> Element {
    let mut registry = use_signal(|| Option::<ProblemRegistry>::None);
    let mut selected_course = use_signal(|| Option::<String>::None);
    let mut selected_chapter = use_signal(|| Option::<String>::None);
    let mut selected_topic = use_signal(|| Option::<String>::None);

    use_effect(move || {
        wasm_bindgen_futures::spawn_local(async move {
            if let Ok(data) = backend::load_registry().await {
                registry.set(Some(data));
            }
        });
    });

    rsx! {
        div { id: "topic-picker",
            h1 { "Topic picker" }
            if let Some(reg) = registry.read().as_ref() {
                select {
                    onchange: move |ev| {
                        selected_course.set(Some(ev.value().to_string()));
                        selected_chapter.set(None);
                        selected_topic.set(None);
                    },
                    option { value: "", "Select Course" }
                    {reg.courses.iter().map(|course| rsx! {
                        option { value: "{course.name}", "{course.name}" }
                    })}
                }

                // Chapters
                if let Some(course_name) = selected_course.read().as_ref() {
                    if let Some(course) = reg.courses.iter().find(|c| &c.name == course_name) {
                        select {
                            onchange: move |ev| {
                                selected_chapter.set(Some(ev.value().to_string()));
                                selected_topic.set(None);
                            },
                            option { value: "", "Select Chapter" }
                            {course.chapters.iter().map(|chapter| rsx! {
                                option { value: "{chapter.name}", "{chapter.name}" }
                            })}
                        }

                        // Topics
                        if let Some(chapter_name) = selected_chapter.read().as_ref() {
                            if let Some(chapter) = course.chapters.iter().find(|ch| &ch.name == chapter_name) {
                                select {
                                    onchange: move |ev| {
                                        selected_topic.set(Some(ev.value().to_string()));
                                    },
                                    option { value: "", "Select Topic" }
                                    {chapter.topics.iter().map(|topic| rsx! {
                                        option { value: "{topic.name}", "{topic.name}" }
                                    })}
                                }

                                // Problems
                                if let Some(topic_name) = selected_topic.read().as_ref() {
                                    if let Some(topic) = chapter.topics.iter().find(|t| &t.name == topic_name) {
                                        ul {
                                            {topic.problems.iter().map(|problem| rsx! {
                                                li { "{problem}" }
                                            })}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
