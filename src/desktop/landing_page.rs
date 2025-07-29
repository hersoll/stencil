use dioxus::prelude::*;

#[component]
pub fn LandingPage() -> Element {
    rsx! {
        p { "This is the landing page." }
    }
}
