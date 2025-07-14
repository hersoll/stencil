//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]

use app::{TRANSLATIONS, backend, errors};
use dioxus::prelude::*;

use app::components::{CourseSelection, ErrorDisplay, Header, PDFButtons};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    //use_context_provider(|| Signal::new(SetData(Vec::new())));
    let translations = use_server_future(backend::load_translations)?;
    if let Some(Err(e)) = translations() {
        return rsx! { "Error loading translations: {e}"};
    }
    *TRANSLATIONS.write() = translations().unwrap().unwrap();

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
            CourseSelection {}
            PDFButtons {}
        }
    }
}
