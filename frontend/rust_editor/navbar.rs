use dioxus::prelude::*;

use crate::editor::router::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { class: "navbar",
            ul {
                li {
                    Link { class: "navbar_item", to: Route::CoursePage {}, "Course" }
                }
                li {
                    Link { class: "navbar_item", to: Route::ChapterPage {}, "Chapter" }
                }
                li {
                    Link { class: "navbar_item", to: Route::TopicPage {}, "Topic" }
                }
                li {
                    Link { class: "navbar_item", to: Route::ProblemPage {}, "Problem" }
                }
            }
        }
        Outlet::<Route> {}
    }
}
