mod course_attributes;
mod course_display;
use course_attributes::CourseAttributes;
use course_display::CourseDisplay;

use dioxus::prelude::*;

use crate::shared::CourseData;

#[component]
pub fn CourseEditor() -> Element {
    let active_course: Signal<Option<CourseData>> = use_signal(|| None);
    rsx! {
        div { class: "editor_container",
            CourseDisplay { active_course }
            CourseAttributes { active_course }
        }
    }
}
