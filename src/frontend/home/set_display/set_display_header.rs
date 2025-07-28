use crate::frontend::i18n_lookup;
use dioxus::prelude::*;

#[component]
pub fn SetDisplayHeader() -> Element {
    rsx! {
        div { class: "set_display_legend",
            p { class: "set_display_legend_topics", "{i18n_lookup(\"topics\")?}:" }
            p { class: "set_display_legend_difficulty", "{i18n_lookup(\"difficulty\")?}:" }
            p { class: "set_display_legend_number", "{i18n_lookup(\"number_of_problems\")?}:" }
            p { "" }
        }
    }
}
