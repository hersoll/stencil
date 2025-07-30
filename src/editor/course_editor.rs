mod course_attributes;
mod course_display;
use course_attributes::CourseAttributes;
use course_display::CourseDisplay;

use dioxus::prelude::*;

#[component]
pub fn CourseEditor() -> Element {
    rsx! {
        div { class: "editor_container",
            div { class: "pane available",
                CourseDisplay {}
                div {
                    class: "button_containter",
                    style: "display: flex; gap: 1rem;",
                    button { class: "button", "Skapa ny" }
                    button { class: "button", "Redigera" }
                }
            }
            CourseAttributes {}
        }
    }
}
