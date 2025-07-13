//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]

use app::backend::Translations;
use app::{backend, errors, TRANSLATIONS};
use dioxus::prelude::*;

use app::components::{CourseSelection, ErrorDisplay, Header, LanguageSwitch, PDFButtons};
use app::frontend_types::SetData;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}


#[component]
fn App() -> Element {
    use_context_provider(|| Signal::new(SetData(Vec::new())));
    let translation_fetch_result: Resource<Result<Translations, String>> =
        use_server_future(move || async move {
            let translations = backend::load_translations()
                .await
                .map_err(|err| err.to_string())?;
            Ok(translations)
        })?;
    // We want this to panic if we don't get our translations
    // TODO: Send user to some error page instead of panic
    *TRANSLATIONS.write() = translation_fetch_result().unwrap().unwrap();

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
            LanguageSwitch {}
            CourseSelection {}
            PDFButtons {}
        }
    }
}
