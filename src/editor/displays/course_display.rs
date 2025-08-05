use dioxus::prelude::*;

use crate::shared::CourseData;

#[component]
pub fn CourseDisplay(
    selected_course: Signal<Option<CourseData>>,
    courses: Signal<Vec<CourseData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "available_element course header",
            style: "position: relative;",
            p { "Name" }
            p { "Swedish" }
            p { "English" }
        }
        for course in courses() {
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
