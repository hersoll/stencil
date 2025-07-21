//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]

use app::{
    DocumentOptions, TRANSLATIONS,
    backend::{self, ChapterData, CourseData, TopicData},
    components::{
        CreateSet, DifficultyPicker, NumberPicker, SetDisplay, SetOptions, ToolTipDisplay,
    },
    errors,
    frontend_types::{ProblemSetData, Sets},
};
use dioxus::prelude::*;

use app::components::{ErrorDisplay, Header, PDFButtons, ProblemDisplay};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let options = use_signal(|| DocumentOptions::default());

    let mut set_data = use_signal(|| ProblemSetData::new(0));
    let mut sets: Signal<Sets> = use_signal(|| Vec::new());

    let mut courses: Signal<Vec<CourseData>> = use_signal(|| Vec::new());
    let chapters: Signal<Vec<ChapterData>> = use_signal(|| Vec::new());
    let topics: Signal<Vec<TopicData>> = use_signal(|| Vec::new());

    let active_course = use_signal(|| String::new());
    let active_chapter = use_signal(|| String::new());

    let translations = use_server_future(backend::load_translations)?;
    if let Some(Err(e)) = translations() {
        return rsx! { "Error loading translations: {e}" };
    }
    let registry = use_server_future(backend::load_registry)?;
    if let Some(Err(e)) = registry() {
        return rsx! { "Error loading registry: {e}" };
    }
    *TRANSLATIONS.write() = translations().unwrap().unwrap();
    courses.set(registry().unwrap().unwrap().courses);

    let push_set = move || {
        let set_signal = Signal::new(set_data().clone());
        sets.push(set_signal);
        set_data.write().key += 1;
    };

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
            PDFButtons { sets, options }
        }
    }
}
