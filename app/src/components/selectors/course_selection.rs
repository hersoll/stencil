use crate::APP_LANGUAGE;
use crate::Error;
use crate::backend::TopicData;
use crate::backend::{ChapterData, CourseData, HasDesc};
use dioxus::prelude::*;

fn course_buttons(
    names: Vec<&'static str>,
    group_number: u8,
    courses: Vec<CourseData>,
    mut active_course: Signal<String>,
    mut chapters: Signal<Vec<ChapterData>>,
    mut active_chapter: Signal<String>,
    mut topics: Signal<Vec<TopicData>>,
) -> Element {
    let class_string = format!("course_{group_number}");
    rsx! {
        div { class: "course_group {class_string}",
            for course_name in names {
                {
                    let selected = if active_course() == course_name.to_string() {
                        "selected"
                    } else {
                        ""
                    };
                    let course = courses
                        .iter()
                        .find(|c| c.name == course_name)
                        .ok_or(Error::NoCourseWithCourseName {
                            name: course_name.to_string(),
                        })?
                        .clone();
                    let course_desc = &course.get_desc(APP_LANGUAGE())?;
                    rsx! {
                        button {
                            key: "{course_name.clone()}",
                            class: "course {selected}",
                            onclick: move |_| {
                                topics.set(Vec::new());
                                active_chapter.set(String::new());
                                active_course.set(course_name.to_string());
                                chapters.set(course.chapters.clone());
                            },
                            "{course_desc}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CourseSelection(
    courses: Signal<Vec<CourseData>>,
    mut active_course: Signal<String>,
    chapters: Signal<Vec<ChapterData>>,
    mut active_chapter: Signal<String>,
    topics: Signal<Vec<TopicData>>,
) -> Element {
    //let heading = TRANSLATIONS().get_phrase("site_heading", APP_LANGUAGE())?;
    //let selection_default = TRANSLATIONS().get_phrase("course_selector", APP_LANGUAGE())?;

    rsx! {

        if courses().len() > 0 {
            div { class: "courses",
                {
                    course_buttons(
                        vec!["ma1a", "ma1b", "ma1c"],
                        1,
                        courses(),
                        active_course,
                        chapters,
                        active_chapter,
                        topics,
                    )
                }
                {
                    course_buttons(
                        vec!["ma2a", "ma2b", "ma2c"],
                        2,
                        courses(),
                        active_course,
                        chapters,
                        active_chapter,
                        topics,
                    )
                }
                        //{course_buttons(vec!["ma3b","ma3c"], 3, courses(), chapters)}
            //{course_buttons(vec!["ma4"], 4, courses(), chapters)}
            //{course_buttons(vec!["ma5"], 5, courses(), chapters)}
            }
        }
    }
}
