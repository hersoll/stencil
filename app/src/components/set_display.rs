mod set_display_editing_panel;
mod set_display_exclusions;
mod set_display_header;
mod set_display_options;
mod set_display_row;

use crate::backend::CourseData;
use crate::components::set_display::set_display_header::SetDisplayHeader;
use crate::components::set_display::set_display_row::SetDisplayRow;
use crate::i18n_lookup;
use dioxus::prelude::*;

use crate::frontend_types::Sets;

#[component]
pub fn SetDisplay(sets: Signal<Sets>, courses: Signal<Vec<CourseData>>) -> Element {
    // Descriptions to show in the topic display (first column) before changing to +N
    let max_descriptions = 2;

    rsx! {
        SetDisplayHeader {}
        div { class: "set_display",
            if sets().len() == 0 {
                div { class: "set_row_placeholder",
                    p { "{i18n_lookup(\"please_create_a_set\")?}" }
                    p {}
                    p {}
                    p {}
                }
            }
            // set is still a signal
            for (index , set) in sets().iter().enumerate() {
                SetDisplayRow {
                    key: "{set().key}",
                    sets,
                    index,
                    max_descriptions,
                }
            }
        }
    }
}
