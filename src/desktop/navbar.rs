use dioxus::prelude::*;

use crate::desktop::router::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar",
            ul {
                li {
                    Link { class: "navbar_item", to: Route::CourseEditor {}, "Kurs" }
                }
                li {
                    Link { class: "navbar_item", to: Route::ChapterEditor {}, "Kapitel" }
                }
                li {
                    Link { class: "navbar_item", to: Route::TopicEditor {}, "Område" }
                }
                li {
                    Link { class: "navbar_item", to: Route::ProblemEditor {}, "Problem" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
