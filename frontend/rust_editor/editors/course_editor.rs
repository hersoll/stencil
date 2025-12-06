use dioxus::prelude::*;

use crate::{
    api,
    editor::displays::ChapterDisplay,
    shared::{ChapterData, CourseData},
};

#[component]
pub fn CourseEditor(
    active_course: Signal<Option<CourseData>>,
    selected_chapter: Signal<Option<ChapterData>>,
    used_chapters: Signal<Vec<ChapterData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div { class: "pane attributes",
            h2 { "Attributes" }
            label {
                "Name"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_course().is_some() { false } else { true },
                    onchange: move |event| {
                        active_course
                            .with_mut(|course_opt| {
                                if let Some(course) = course_opt {
                                    course.name = event.value();
                                }
                            });
                    },
                    value: if let Some(course) = active_course() { course.name },
                }
            }
            label {
                "Description (sv)"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_course().is_some() { false } else { true },
                    onchange: move |event| {
                        active_course
                            .with_mut(|course_opt| {
                                if let Some(course) = course_opt {
                                    course.desc_sv = event.value();
                                }
                            });
                    },
                    value: if let Some(course) = active_course() { course.desc_sv },
                }
            }
            label {
                "Description (en)"
                input {
                    class: "input text",
                    r#type: "text",
                    disabled: if active_course().is_some() { false } else { true },
                    onchange: move |event| {
                        active_course
                            .with_mut(|course_opt| {
                                if let Some(course) = course_opt {
                                    course.desc_en = event.value();
                                }
                            });
                    },
                    value: if let Some(course) = active_course() { course.desc_en },
                }
            }

            label {
                "Chapters"
                div { class: "available_display", style: "height: 400px;",
                    ChapterDisplay {
                        chapters: used_chapters,
                        selected_chapter,
                        current_message,
                    }
                }
            }
            div {
                class: "button_container",
                style: "display: flex; gap: 1rem; justify-content: center;",
                button {
                    class: "button",
                    style: "width: 15rem;",
                    onclick: move |_| async move {
                        if let Some(course) = active_course() {
                            match api::set_course(course.clone()).await {
                                Ok(saved) => {
                                    match api::set_course_chapters(course.id, used_chapters().clone())
                                        .await
                                    {
                                        Ok(_) => {
                                            current_message
                                                .set(Some(format!("Saved course:\n {:#?}", saved)))
                                        }
                                        Err(message) => current_message.set(Some(message.to_string())),
                                    }
                                }
                                Err(message) => current_message.set(Some(message.to_string())),
                            }
                        }
                        active_course.set(None);
                    },
                    "Save"
                }
                button {
                    class: "button",
                    style: "width: 8rem;",
                    onclick: move |_| {
                        active_course.set(None);
                    },
                    "Undo"
                }
            }
        }
    }
}
