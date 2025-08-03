use dioxus::prelude::*;

use crate::{
    api::{self, load_all_chapter_data},
    shared::CourseData,
};

#[component]
pub fn CourseEditor(
    active_course: Signal<Option<CourseData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    let loaded_chapters = use_server_future(move || load_all_chapter_data())?;
    let mut related_chapters: Signal<Vec<i32>> = use_signal(|| Vec::new());
    let chapter_resource = use_resource(move || async move {
        if let Some(course) = active_course() {
            let active_data = crate::api::load_course_chapters(course.id).await;
            if let Ok(chapters) = active_data {
                return chapters;
            }
        }
        Vec::new()
    });
    use_effect(move || {
        if let Some(loaded_chapters) = chapter_resource() {
            related_chapters.set(loaded_chapters);
        }
    });

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
                    match loaded_chapters().unwrap() {
                        Ok(chapters) => {
                            rsx! {
                                for chapter in chapters {
                                    div {
                                        class: "available_element chapter item",
                                        style: if related_chapters().contains(&chapter.id) { "background-color: gray;" } else { "" },
                                        onclick: move |_| {
                                            if let Some(pos) = related_chapters().iter().position(|id| *id == chapter.id) {
                                                related_chapters.remove(pos);
                                            } else {
                                                related_chapters.write().push(chapter.id)
                                            }
                                        },
                                        p { "{chapter.id}" }
                                        p { "{chapter.name}" }
                                        p { "{chapter.desc_sv}" }
                                        p { "{chapter.desc_en}" }
                                    }
                                }
                            }
                        }
                        Err(message) => {
                            current_message.set(Some(message.to_string()));
                            rsx! {}
                        }
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
                            match api::set_course(course).await {
                                Ok(saved) => {
                                    current_message.set(Some(format!("Saved course:\n {:#?}", saved)))
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
                    onclick: move |_| active_course.set(None),
                    "Undo"
                }
            }
        }
    }
}
