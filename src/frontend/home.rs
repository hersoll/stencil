mod create_set;
mod difficulty_picker;
mod document_options;
mod error_display;
mod header;
mod language_switch;
mod number_picker;
mod pdf_button;
mod selectors;
mod set_display;
mod set_options;
mod toggle;
mod tooltip;

pub use create_set::CreateSet;
pub use difficulty_picker::DifficultyPicker;
pub use document_options::DocumentOptionDisplay;
pub use error_display::ErrorDisplay;
pub use header::Header;
pub use language_switch::LanguageSwitch;
pub use number_picker::NumberPicker;
pub use pdf_button::PDFButtons;
pub use selectors::problem_display::ProblemDisplay;
pub use set_display::SetDisplay;
pub use set_options::SetOptions;
pub use tooltip::ToolTip;
pub use tooltip::ToolTipDisplay;

use dioxus::prelude::*;

use crate::api;
use crate::frontend::*;
use crate::shared;

#[component]
pub fn Home() -> Element {
    let options = use_signal(|| shared::DocumentOptions::default());

    let mut set_data = use_signal(|| shared::ProblemSetData::new(0));
    let mut sets: Signal<Sets> = use_signal(|| Vec::new());

    let mut courses: Signal<Vec<shared::ParsedCourseData>> = use_signal(|| Vec::new());
    let chapters: Signal<Vec<i32>> = use_signal(|| Vec::new());
    let topics: Signal<Vec<i32>> = use_signal(|| Vec::new());

    let active_course: Signal<i32> = use_signal(|| -1);
    let active_chapter: Signal<i32> = use_signal(|| -1);

    let course_result = use_server_future(move || api::load_courses(APP_LANGUAGE()))?;
    if let Some(Err(e)) = course_result() {
        return rsx! { "Error loading courses: {e}" };
    }
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
        Header {}
        ToolTipDisplay {}
        ErrorBoundary {
            handle_error: |error: ErrorContext| {
                rsx! {
                    for e in error.error() {
                        ErrorDisplay { message_signal: crate::clean_error_message(format!("{:#?}", e)) }
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
