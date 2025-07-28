use crate::{
    api,
    frontend::{APP_LANGUAGE, Route, TRANSLATIONS},
};
use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

// Setting the head before spinning up the rest of the app
#[component]
pub fn AppSetup() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Link { rel: "preconnect", href: "https://fonts.gstatic.com" }
        document::Link {
            rel: "stylesheet",
            href: "https://fonts.googleapis.com/css2?family=Oswald:wght@200..700&display=swap",
        }
        document::Style {
            "
            .loading-screen {{
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
                font-family: 'Oswald', system-ui, sans-serif;
                font-size: 1.2rem;
                background-color: #0f1116;
                color: white;
            }}
            "
        }
        App {}
    }
}

#[component]
pub fn App() -> Element {
    let mut translations_loaded = use_signal(|| false);
    let translations = use_server_future(move || api::load_translations(APP_LANGUAGE()))?;
    if let Some(Err(e)) = translations() {
        return rsx! { "Error loading translations: {e}" };
    }
    use_effect(move || {
        if let Some(Ok(translation_data)) = translations() {
            *TRANSLATIONS.write() = translation_data;
            translations_loaded.set(true);
        }
    });
    rsx! {
        // Don't render main content until translations are loaded
        if !translations_loaded() {
            div { class: "loading-screen", "Loading application..." }
        } else {

            Router::<Route> {}
        }
    }
}
