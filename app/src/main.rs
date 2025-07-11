//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]
use dioxus::prelude::*;

use app::components::{Header, TopicSelection, PDFButtons};
use app::frontend_types::SetData;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| SetData(Vec::new()));

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Oswald:wght@200..700&display=swap",
        }


        Header {}
        TopicSelection {}
        PDFButtons {}
    }
}
