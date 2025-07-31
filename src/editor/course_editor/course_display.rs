use dioxus::prelude::*;

use crate::{api::{self, load_all_course_data}, shared::CourseData};

#[component]
pub fn CourseDisplay(active_course: Signal<Option<CourseData>>) -> Element {
    let mut courses_result = use_server_future(move || load_all_course_data())?;
    let mut selected_course: Signal<Option<CourseData>> = use_signal(move || None);
    rsx! {
        div { class: "pane available",
            div { class: "available_display", style: "height: 760px;",
                div { class: "available_element course header",
                    style: "position: relative;",
                    p { "ID" }
                    p { "Namn" }
                    p { "Svenska" }
                    p { "Engelska" }
                    div {class: "update_arrow",
                    onclick: move |_| courses_result.restart(),
                    "{char::from_u32(0x27F3).unwrap()}"}
                }
                if let Ok(courses) = courses_result().unwrap() {
                    for course in courses {
                        div {
                            class: "available_element course item",
                            style: if let Some(selected) = selected_course() { if selected.id == course.id { "background-color: gray;" } else { "" } },
                            onclick: move |_| selected_course.set(Some(course.clone())),
                            p { "{course.id}" }
                            p { "{course.name}" }
                            p { "{course.desc_sv}" }
                            p { "{course.desc_en}" }
                        }
                    }
                } else {
                    p { "Loading..." }
                }
            }
            div { class: "button_container", style: "display: flex; gap: 1rem;",
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
                    "Skapa ny"
                }
                button {
                    class: "button",
                    onclick: move |_| {
                        if let Some(course) = selected_course() {
                            active_course.set(Some(course))
                        }
                    },
                    "Redigera"
                }
                 button {
                    class: "button",
                    onclick: move |_| async move {
                        if let Some(course) = selected_course() {
                                api::delete_course(course.id).await;
                        }
                    },
                    "Radera"
                }
            }
        }
    }
}
