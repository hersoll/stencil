use crate::{
    components::selectors::course_selection::CourseSelection, frontend_types::ProblemSetData,
};
use dioxus::prelude::*;

use crate::{
    backend::{ChapterData, CourseData, TopicData},
    components::selectors::{chapter_selection::ChapterSelection, topic_selection::TopicSelection},
};

#[component]
pub fn ProblemDisplay(
    set_data: Signal<ProblemSetData>,
    courses: Signal<Vec<CourseData>>,
    chapters: Signal<Vec<ChapterData>>,
    topics: Signal<Vec<TopicData>>,
    active_course: Signal<String>,
    active_chapter: Signal<String>,
) -> Element {
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
                    TopicSelection { topics, set_data }
                }
            }
        }
    }
}
