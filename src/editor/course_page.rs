use crate::editor::displays::CourseDisplay;
use crate::editor::editors::CourseEditor;
use crate::{api, editor::displays::ChapterDisplay};
use dioxus::prelude::*;

use crate::shared::{ChapterData, CourseData};

#[component]
pub fn CoursePage() -> Element {
    let mut course_future = use_server_future(move || api::load_all_course_data())?;
    let chapter_future = use_server_future(move || api::load_all_chapter_data())?;
    let mut active_course: Signal<Option<CourseData>> = use_signal(|| None);
    let mut selected_course: Signal<Option<CourseData>> = use_signal(|| None);
    let selected_chapter: Signal<Option<ChapterData>> = use_signal(|| None);
    let mut current_message: Signal<Option<String>> = use_signal(|| None);

    use_effect(move || {
        if active_course().is_none() {
            selected_course.set(None);
        }
    });

    let chapter_resource = use_resource(move || async move {
        if let Some(course) = active_course() {
            let active_data = crate::api::load_course_chapters(course.id).await;
            if let Ok(chapters) = active_data {
                return chapters;
            }
        }
        Vec::new()
    });
    let mut course_chapters: Signal<Vec<i32>> = use_signal(|| Vec::new());
    use_effect(move || {
        if let Some(loaded_chapters) = chapter_resource() {
            course_chapters.set(loaded_chapters);
        }
    });

    rsx! {
        div { class: "editor_container",
            div { class: "pane available",
                div { class: "available_display", style: "height: 760px;",
                    if let Some(message) = current_message() {
                        div { style: "padding: 1rem; font-weight: 200;", "{message}" }
                    } else if active_course().is_some() {
                        ChapterDisplay {
                            selected_chapter,
                            chapter_future,
                            current_message,
                        }
                    } else {
                        CourseDisplay {
                            selected_course,
                            course_future,
                            current_message,
                        }
                    }
                }
                div {
                    class: "button_container",
                    style: "display: flex; gap: 1rem;",
                    if current_message().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                course_future.restart();
                                current_message.set(None);
                            },
                            "OK"
                        }
                    } else {
                        button {
                            class: "button",
                            onclick: move |_| {
                                active_course
                                    .set(
                                        Some(CourseData {
                                            id: 0,
                                            name: String::new(),
                                            desc_sv: String::new(),
                                            desc_en: String::new(),
                                        }),
                                    );
                                selected_course.set(None);
                            },
                            "Create new"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(course) = selected_course() {
                                    active_course.set(Some(course))
                                }
                            },
                            "Edit"
                        }
                        button {
                            class: "button",
                            onclick: move |_| async move {
                                if let Some(course) = selected_course() {
                                    match api::delete_course(course.id).await {
                                        Ok(deleted) => {
                                            current_message
                                                .set(Some(format!("Deleted course: \n {:#?}", deleted)))
                                        }
                                        Err(message) => current_message.set(Some(message.to_string())),
                                    }
                                }
                            },
                            "Delete"
                        }
                    }
                }
            }
            CourseEditor { active_course, current_message }
        }
    }
}
