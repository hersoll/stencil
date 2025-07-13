//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]
use app::errors;
use dioxus::prelude::*;

use app::components::{ErrorDisplay, Header, PDFButtons, TopicSelection};
use app::frontend_types::{Language, SetData};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    use_context_provider(|| SetData(Vec::new()));
    use_context_provider(|| Signal::new(Language("en".to_string())));

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
        ErrorBoundary {
            handle_error: |error: ErrorContext| {
                rsx! {
                    for e in error.errors() {
                        ErrorDisplay { message_signal: errors::clean_error_message(format!("{:#?}", e)) }
                    }
                }
            },
            TopicSelection {}
            PDFButtons {}
        }
    }
}
