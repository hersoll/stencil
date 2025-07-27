use dioxus::prelude::*;

use crate::api;
use crate::frontend::*;
use crate::frontend::{ErrorDisplay, Header, PDFButtons, ProblemDisplay};
use crate::shared;
use crate::shared::errors;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

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
    let options = use_signal(|| shared::DocumentOptions::default());

    let mut set_data = use_signal(|| shared::ProblemSetData::new(0));
    let mut sets: Signal<Sets> = use_signal(|| Vec::new());

    let mut courses: Signal<Vec<shared::CourseInfo>> = use_signal(|| Vec::new());
    let chapters: Signal<Vec<i32>> = use_signal(|| Vec::new());
    let topics: Signal<Vec<i32>> = use_signal(|| Vec::new());

    let active_course: Signal<i32> = use_signal(|| -1);
    let active_chapter: Signal<i32> = use_signal(|| -1);

    let translations = use_server_future(move || api::load_translations(APP_LANGUAGE()))?;
    let course_result = use_server_future(move || api::load_courses(APP_LANGUAGE()))?;
    if let Some(Err(e)) = translations() {
        return rsx! { "Error loading translations: {e}" };
    }
    if let Some(Err(e)) = course_result() {
        return rsx! { "Error loading courses: {e}" };
    }
    use_effect(move || {
        if let Some(Ok(translation_data)) = translations() {
            *TRANSLATIONS.write() = translation_data;
            translations_loaded.set(true);
        }
    });
    use_effect(move || {
        if let Some(Ok(course_data)) = course_result() {
            *courses.write() = course_data;
        }
    });

    let push_set = move || {
        let set_signal = Signal::new(set_data().clone());
        sets.push(set_signal);
        set_data.write().key += 1;
    };

    rsx! {
        // Don't render main content until translations are loaded
        if !translations_loaded() {
            div { class: "loading-screen", "Loading application..." }
        } else {


            Header {}
            ToolTipDisplay {}
            ErrorBoundary {
                handle_error: |error: ErrorContext| {
                    rsx! {
                        for e in error.errors() {
                            ErrorDisplay { message_signal: errors::clean_error_message(format!("{:#?}", e)) }
                        }
                    }
                },
                ProblemDisplay {
                    set_data,
                    courses,
                    chapters,
                    topics,
                    active_course,
                    active_chapter,
                }
                SetOptions { set_data }
                CreateSet { set_data, sets, set_pusher: push_set }
                SetDisplay { sets, courses }
                DocumentOptionDisplay { options }
                PDFButtons { sets, options }
            }
        }
    }
}
