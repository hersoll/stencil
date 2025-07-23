//Remember to eventually remove this (when #[server] is fixed....)
#![allow(dead_code)]

use app::api::load_courses;
use app::{api::load_translations, frontend::*};
use app::shared::errors;
use app::shared::*;
use dioxus::prelude::*;

use app::frontend::{ErrorDisplay, Header, PDFButtons, ProblemDisplay};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "web")]
    dioxus::launch(App);

    #[cfg(feature = "server")]
    {
        use sqlx::Postgres;

        dotenv::dotenv().ok();

        // Create database pool once
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = sqlx::Pool<Postgres>::connect(&database_url).await?;

        // Store pool in Dioxus server context
        dioxus_fullstack::launch::LaunchBuilder::new(app)
            .with_context(pool) // Share the pool with all server functions
            .launch()
            .await;
    }
}

#[component]
fn App() -> Element {
    let options = use_signal(|| DocumentOptions::default());

    let mut set_data = use_signal(|| ProblemSetData::new(0));
    let mut sets: Signal<Sets> = use_signal(|| Vec::new());

    let mut courses: Signal<Vec<CourseInfo>> = use_signal(|| Vec::new());
    let chapters: Signal<Vec<ChapterInfo>> = use_signal(|| Vec::new());
    let topics: Signal<Vec<TopicInfo>> = use_signal(|| Vec::new());

    let active_course = use_signal(|| String::new());
    let active_chapter = use_signal(|| String::new());

    let translations = use_server_future(move || load_translations(APP_LANGUAGE()))?;
    if let Some(Err(e)) = translations() {
        return rsx! { "Error loading translations: {e}" };
    }
    *TRANSLATIONS.write() = translations().unwrap().unwrap();
    courses.write() = use_server_future(move || load_courses(APP_LANGUAGE()))?.unwrap().unwrap();

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
            DocumentOptionDisplay { options }
            PDFButtons { sets, options }
        }
    }
}
