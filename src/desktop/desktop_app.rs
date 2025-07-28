use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn DesktopApp() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        h1 { "Hello to the desktop! This is cool!" }
    }
}
