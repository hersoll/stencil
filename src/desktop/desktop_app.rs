use dioxus::prelude::*;

const DESKTOP_CSS: Asset = asset!("/assets/styling/desktop.css");

#[component]
pub fn DesktopApp() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DESKTOP_CSS }

        h1 { "Stencil editor" }
    }
}
