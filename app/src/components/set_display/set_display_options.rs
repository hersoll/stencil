mod set_option_columns;
mod set_option_spacing;
mod set_option_title;
use set_option_columns::SetOptionColumns;
use set_option_spacing::SetOptionSpacing;
use set_option_title::SetOptionTitle;

use crate::frontend_types::Sets;
use dioxus::prelude::*;

#[component]
pub fn SetDisplayOptions(sets: Signal<Sets>, index: usize) -> Element {
    let set = sets()[index];
    rsx! {
        div { class: "set_options",
            SetOptionTitle { set }
            SetOptionColumns { set }
            SetOptionSpacing { set }
        }
    }
}
