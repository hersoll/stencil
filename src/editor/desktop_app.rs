use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[component]
pub fn DesktopApp() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Oswald:wght@200..700&display=swap",
        }

        p { "Hello to the desktop! This is cool!" }
    }
}
