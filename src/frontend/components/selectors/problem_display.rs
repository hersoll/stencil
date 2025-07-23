use crate::{
    frontend::components::selectors::course_selection::CourseSelection,
    shared::types::ProblemSetData,
};
use dioxus::prelude::*;

use crate::{
    frontend::components::selectors::{
        chapter_selection::ChapterSelection, topic_selection::TopicSelection,
    },
    shared::{ChapterInfo, CourseInfo, TopicInfo},
};

#[component]
pub fn ProblemDisplay(
    set_data: Signal<ProblemSetData>,
    courses: Signal<Vec<CourseInfo>>,
    chapters: Signal<Vec<ChapterInfo>>,
    topics: Signal<Vec<TopicInfo>>,
    active_course: Signal<i32>,
    active_chapter: Signal<i32>,
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
                if active_course() >= 0 {
                    ChapterSelection { chapters, active_chapter, topics }
                } else {
                    div { class: "display_placeholder", "Välj en kurs" }
                }
            }
            div { class: "chapter_separator" }
            div { class: "topics",
                if active_course() < 0 {
                    div { class: "display_placeholder", "Välj en kurs" }
                } else if active_chapter() < 0{
                    div { class: "display_placeholder", "Välj ett kapitel" }
                } else {
                    TopicSelection { topics, set_data }
                }
            }
        }
    }
}
