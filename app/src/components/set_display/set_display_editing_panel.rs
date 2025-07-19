use crate::{
    components::set_display::set_display_options::SetDisplayOptions, frontend_types::Sets,
    i18n_lookup,
};
use dioxus::prelude::*;

use crate::components::set_display::set_display_exclusions::ProblemExclusions;

#[component]
pub fn SetDisplayEditingPanel(sets: Signal<Sets>, index: usize, editing: Signal<bool>) -> Element {
    let set = sets()[index];
    rsx! {
        div { class: if editing() { "editing_display editing" } else { "editing_display" },
            div { class: "editing_content_wrapper",
                p { class: "editing_header exclusions", "{i18n_lookup(\"choose_exclusions\")?}:" }
                p { class: "editing_header options", "{i18n_lookup(\"options\")?}:" }

                ProblemExclusions { key: "{set().key}", sets, index }
                SetDisplayOptions { key: "{set().key}", sets, index }
            }
        }
    }
}
