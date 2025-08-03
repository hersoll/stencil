use dioxus::prelude::*;

use crate::{editor::displays::update_arrow::UpdateArrow, shared::CourseData};

#[component]
pub fn CourseDisplay(
    selected_course: Signal<Option<CourseData>>,
    course_future: Resource<Result<Vec<CourseData>, ServerFnError>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "available_element course header",
            style: "position: relative;",
            p { "Namn" }
            p { "Svenska" }
            p { "Engelska" }
            UpdateArrow { future: course_future }
        }
        match course_future().unwrap() {
            Ok(courses) => {
                rsx! {
                    for course in courses {
                        div {
                            class: "available_element course item",
                            style: if let Some(selected) = selected_course() { if selected.id == course.id { "background-color: gray;" } else { "" } },
                            onclick: move |_| selected_course.set(Some(course.clone())),
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
