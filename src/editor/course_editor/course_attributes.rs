use dioxus::prelude::*;

#[component]
pub fn CourseAttributes() -> Element {
    rsx! {
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
