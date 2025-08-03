use crate::api::load_course_chapters;
use crate::shared::ParsedCourseData;
use crate::Error;
use dioxus::prelude::*;

fn course_buttons(
    names: Vec<&'static str>,
    group_number: u8,
    courses: Vec<ParsedCourseData>,
    mut chapters: Signal<Vec<i32>>,
    mut active_course: Signal<i32>,
    mut active_chapter: Signal<i32>,
    mut topics: Signal<Vec<i32>>,
) -> Element {
    let class_string = format!("course_{group_number}");
    rsx! {
        div { class: "course_group {class_string}",
            for course_name in names {
                {
                    let course = courses
                        .iter()
                        .find(|c| c.name == course_name)
                        .ok_or(Error::NoCourseWithCourseName {
                            name: course_name.to_string(),
                        })?
                        .clone();
                    let selected = if active_course() == course.id { "selected" } else { "" };
                    rsx! {
                        button {
                            key: "{course_name.clone()}",
                            class: "course {selected}",
                            disabled: course_name != "ma1b",
                            onclick: move |_| async move {
                                topics.set(Vec::new());
                                active_chapter.set(-1);
                                active_course.set(course.id);
                                match load_course_chapters(course.id).await {
                                    Ok(chapter_info) => chapters.set(chapter_info),
                                    Err(_) => chapters.set(Vec::new()),
                                }
                            },
                            "{course.desc}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CourseSelection(
    courses: Signal<Vec<ParsedCourseData>>,
    mut active_course: Signal<i32>,
    chapters: Signal<Vec<i32>>,
    mut active_chapter: Signal<i32>,
    topics: Signal<Vec<i32>>,
) -> Element {
    rsx! {
        if courses().len() > 0 {
            div { class: "courses",
                {
                    course_buttons(
                        vec!["ma1a", "ma1b", "ma1c"],
                        1,
                        courses(),
                        chapters,
                        active_course,
                        active_chapter,
                        topics,
                    )
                }
                {
                    course_buttons(
                        vec!["ma2a", "ma2b", "ma2c"],
                        2,
                        courses(),
                        chapters,
                        active_course,
                        active_chapter,
                        topics,
                    )
                }
            }
        }
    }
}
