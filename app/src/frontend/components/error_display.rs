use dioxus::prelude::*;

#[component]
pub fn ErrorDisplay(message_signal: String) -> Element {
    if !message_signal.is_empty() {
        rsx! {
            p { id: "error_display", "{message_signal}" }
        }
    } else {
        rsx! {}
    }
}
