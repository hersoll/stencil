use dioxus::prelude::*;

#[component]
pub fn CourseEditor() -> Element {
    rsx! {
        div { class: "editor_container",
            div { class: "pane available",
                div { class: "available_display", style: "height: 760px;" }
                div {
                    class: "button_containter",
                    style: "display: flex; gap: 1rem;",
                    button { class: "button", "Skapa ny" }
                    button { class: "button", "Redigera" }
                }
            }
            div { class: "pane attributes",
                h2 { "Egenskaper" }
                label {
                    "Namn"
                    input { class: "input text", r#type: "text" }
                }
                label {
                    "Beskrivning (sv)"
                    input { class: "input text", r#type: "text" }
                }
                label {
                    "Beskrivning (en)"
                    input { class: "input text", r#type: "text" }
                }

                label {
                    "Kapitel"
                    div { class: "available_display", style: "height: 400px;" }
                }

                button { class: "button", "Spara" }
            }
        }
    }
}
