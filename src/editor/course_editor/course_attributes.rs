use dioxus::prelude::*;

use crate::{
    api::{self, load_all_chapter_data},
    shared::CourseData,
};

#[component]
pub fn CourseAttributes(active_course: Signal<Option<CourseData>>) -> Element {
    let loaded_chapters = use_server_future(move || load_all_chapter_data())?;
    let mut related_chapters: Signal<Vec<i32>> = use_signal(|| Vec::new());
    let chapter_resource = use_resource(move || async move {
        if let Some(course) = active_course() {
            let active_data = crate::api::load_course_chapters(course.id, String::from("sv")).await;
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
            h2 { "Egenskaper" }
            label {
                "Namn"
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
                "Beskrivning (sv)"
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
                "Beskrivning (en)"
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
                "Kapitel"
                div { class: "available_display", style: "height: 400px;",
                    if let Ok(chapters) = loaded_chapters().unwrap() {
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
            }

            button { class: "button",
            onclick: move |_| async move {
                if let Some(course) = active_course() {
                    let result = api::set_course(course).await;
                }
            },
            "Spara" }

        }
    }
}
