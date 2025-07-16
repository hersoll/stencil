use crate::components::selectors::course_selection::CourseSelection;
use dioxus::prelude::{server_fn::error::ServerFnErrorErr, *};

use crate::{
    backend::{self, ChapterData, CourseData, ProblemRegistry, TopicData},
    components::selectors::{chapter_selection::ChapterSelection, topic_selection::TopicSelection},
};

#[component]
pub fn ProblemDisplay() -> Element {
    let registry_result: Resource<Result<ProblemRegistry, ServerFnErrorErr>> =
        use_resource(move || async move {
            let reg = backend::load_registry()
                .await
                .map_err(ServerFnErrorErr::from)?;
            Ok(reg)
        });

    let mut courses: Signal<Vec<CourseData>> = use_signal(|| Vec::new());
    let chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());
    let topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());

    let active_course: Signal<String> = use_signal(|| String::new());
    let active_chapter = use_signal(|| <String>::new());
    let active_topic = use_signal(|| <String>::new());

    use_effect(move || {
        if let Some(Ok(registry)) = registry_result() {
            courses.set(registry.courses);
        } else if let Some(Err(e)) = registry_result() {
            throw_error(e.clone())
        }
    });

    rsx! {
        CourseSelection {
            courses,
            active_course,
            chapters,
            active_chapter,
            topics,
        }
        div { class: "problem_display",
            div { class: "chapters",
                if !active_course().is_empty() {
                    ChapterSelection { chapters, active_chapter, topics }
                } else {
                    div { class: "display_placeholder", "Välj en kurs" }
                }
            }
            div { class: "chapter_separator" }
            div { class: "topics",
                if active_course().is_empty() {
                    div { class: "display_placeholder", "Välj en kurs" }
                } else if active_chapter().is_empty() {
                    div { class: "display_placeholder", "Välj ett kapitel" }
                } else {
                    TopicSelection { topics, active_topic }
                }
            }
        }
    }
}
