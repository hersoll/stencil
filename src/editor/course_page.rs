use crate::editor::displays::CourseDisplay;
use crate::editor::editors::CourseEditor;
use crate::{api, editor::displays::ChapterDisplay};
use dioxus::prelude::*;

use crate::shared::{ChapterData, CourseData};

#[component]
pub fn CoursePage() -> Element {
    let mut course_future = use_server_future(move || api::load_all_course_data())?;
    let chapter_future = use_server_future(move || api::load_all_chapter_ids())?;
    let mut active_course: Signal<Option<CourseData>> = use_signal(|| None);
    let mut selected_course: Signal<Option<CourseData>> = use_signal(|| None);
    let mut selected_chapter: Signal<Option<ChapterData>> = use_signal(|| None);
    let mut current_message: Signal<Option<String>> = use_signal(|| None);
    let mut courses: Signal<Vec<CourseData>> = use_signal(|| Vec::new());

    let mut used_chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());
    let mut unused_chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());

    use_effect(move || {
        if active_course().is_none() {
            selected_course.set(None);
            selected_chapter.set(None);
            used_chapters.set(Vec::new());
            unused_chapters.set(Vec::new());
        }
    });

    use_effect(move || match course_future().unwrap() {
        Ok(course_vec) => courses.set(course_vec),
        Err(message) => current_message.set(Some(message.to_string())),
    });

    let _ = use_resource(move || async move {
        if let Some(course) = active_course() {
            match crate::api::load_course_chapters(course.id).await {
                Ok(course_chapters) => match chapter_future().unwrap() {
                    Ok(chapter_ids) => {
                        let used_ids: Vec<i32> = course_chapters.iter().map(|ch| ch.id).collect();
                        let mut unused_ids = chapter_ids.clone();
                        unused_ids.retain(|id| !used_ids.contains(id));
                        match api::load_chapters_by_id(unused_ids).await {
                            Ok(chapters) => unused_chapters.set(chapters),
                            Err(e) => current_message.set(Some(e.to_string())),
                        }
                        used_chapters.set(course_chapters);
                    }
                    Err(e) => current_message.set(Some(e.to_string())),
                },
                Err(e) => current_message.set(Some(e.to_string())),
            }
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
                            chapters: unused_chapters,
                            current_message,
                        }
                    } else {
                        CourseDisplay {
                            selected_course,
                            courses,
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
                    } else if active_course().is_some() {
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(chapter) = selected_chapter() {
                                    if let Some(index) = used_chapters().iter().position(|ch| ch == &chapter) {
                                        used_chapters.write().remove(index);
                                        unused_chapters.write().push(chapter.clone());
                                    } else if let Some(index) = unused_chapters()
                                        .iter()
                                        .position(|ch| ch == &chapter)
                                    {
                                        unused_chapters.write().remove(index);
                                        used_chapters.write().push(chapter.clone());
                                    } else {
                                        current_message
                                            .set(Some(String::from("This button desn't fucking work!")))
                                    }
                                }
                            },
                            "Move"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(chapter) = selected_chapter()
                                    && let Some(index) = used_chapters().iter().position(|ch| ch == &chapter)
                                {
                                    used_chapters.write().remove(index);
                                    used_chapters.write().insert(index - 1, chapter.clone());
                                }
                            },
                            "Up"
                        }
                        button {
                            class: "button",
                            onclick: move |_| {
                                if let Some(chapter) = selected_chapter()
                                    && let Some(index) = used_chapters().iter().position(|ch| ch == &chapter)
                                {
                                    used_chapters.write().remove(index);
                                    used_chapters.write().insert(index + 1, chapter.clone());
                                }
                            },
                            "Down"
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
            CourseEditor {
                active_course,
                current_message,
                used_chapters,
                selected_chapter,
            }
        }
    }
}
