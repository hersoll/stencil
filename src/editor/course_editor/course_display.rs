use dioxus::prelude::*;

use crate::api::{self, load_all_course_data};

#[component]
pub fn CourseDisplay() -> Element {
    let courses_result = use_server_future(move || load_all_course_data())?;
    rsx! {
        div { class: "available_display", style: "height: 760px;",
            div{class: "available_element header",
            p {"ID"}
            p {"Namn"}
            p {"Svenska"}
            p {"Engelska"}
        }
            if let Ok(courses) = courses_result().unwrap() {
                for course in courses {
                div {class: "available_element item",
                    p{"{course.id}"}
                    p{"{course.name}"}
                    p{"{course.desc_sv}"}
                    p{"{course.desc_en}"}
                }
            }
            } else {
                p { "Loading..." }
            }
        }
    }
}
