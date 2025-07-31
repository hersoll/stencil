use dioxus::prelude::*;

use crate::{
    api::{self, load_all_course_data},
    shared::CourseData,
};

#[component]
pub fn CourseDisplay(
    active_course: Signal<Option<CourseData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    let mut courses_result = use_server_future(move || load_all_course_data())?;
    let mut selected_course: Signal<Option<CourseData>> = use_signal(move || None);
    rsx! {
        div { class: "pane available",
            div { class: "available_display", style: "height: 760px;",
                if let Some(message) = current_message() {
                    div { style: "padding: 1rem; font-weight: 200;", "{message}" }
                } else {
                    div {
                        class: "available_element course header",
                        style: "position: relative;",
                        p { "ID" }
                        p { "Namn" }
                        p { "Svenska" }
                        p { "Engelska" }
                        div {
                            class: "update_arrow",
                            onclick: move |_| courses_result.restart(),
                            "{char::from_u32(0x27F3).unwrap()}"
                        }
                    }
                    match courses_result().unwrap() {
                        Ok(courses) => {
                            rsx! {
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
                            }
                        }
                        Err(message) => {
                            current_message.set(Some(message.to_string()));
                            rsx! {}
                        }
                    }
                }
            }
            div { class: "button_container", style: "display: flex; gap: 1rem;",
                if current_message().is_some() {
                    button {
                        class: "button",
                        onclick: move |_| {
                            courses_result.restart();
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
    }
}
