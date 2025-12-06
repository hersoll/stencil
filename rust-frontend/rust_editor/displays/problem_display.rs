use dioxus::prelude::*;

use crate::shared::ProblemData;

#[component]
pub fn ProblemDisplay(
    selected_problem: Signal<Option<ProblemData>>,
    problems: Signal<Vec<ProblemData>>,
    current_message: Signal<Option<String>>,
) -> Element {
    rsx! {
        div {
            class: "available_element problem header",
            style: "position: relative;",
            p { "Module" }
            p { "Name" }
            p { "Swedish" }
        }
        for problem in problems() {
            div {
                class: "available_element problem item",
                style: if let Some(selected) = selected_problem() { if selected.id == problem.id { "background-color: gray;" } else { "" } },
                onclick: move |_| selected_problem.set(Some(problem.clone())),
                p { "{problem.module}" }
                p { "{problem.name}" }
                p { "{problem.desc_sv}" }
            }
        }
    }
}
