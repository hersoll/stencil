use dioxus::prelude::*;

use crate::{
    APP_LANGUAGE, Error,
    backend::{self, CourseData, ProblemData},
    frontend_types::{ProblemSetData, Sets},
};

#[component]
pub fn ProblemExclusions(
    courses: Signal<Vec<CourseData>>,
    sets: Signal<Sets>,
    i: usize,
) -> Element {
    let set = &sets()[i];
    rsx! {
        div { class: "problem_exclusions",
            for problem in courses()
                .iter()
                .flat_map(|course| course.chapters.iter())
                .flat_map(|chapter| chapter.topics.iter())
                .filter(|topic| set.ids.contains(topic))
                .flat_map(|topic| topic.problems.iter())
            {
                {
                    let problem_name = problem.name.clone();
                    let fetch_result = use_resource(move || {
                        let problem_name = problem_name.clone();
                        async move { backend::get_problem_info(problem_name, APP_LANGUAGE()) }
                    });
                    if let Some(desc) = problem.desc.get(APP_LANGUAGE()) {
                        rsx! {
                            div { class: "exclusion_row",
                                p { class: "exclusion_desc", "{desc}" }
                                p { class: "exclusion_difficulty", "{difficulty}" }
                            }
                        }
                    } else {
                        rsx! {
                            div { class: "exclusion_row error",
                                p { "Error: No description available for language {APP_LANGUAGE()}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
