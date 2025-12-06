use crate::frontend::i18n_lookup;
use dioxus::prelude::*;

#[component]
pub fn DocumentOptionsHeader(active: Signal<bool>) -> Element {
    rsx! {
        div {
            class: if active() { "document_options_header active" } else { "document_options_header" },
            onclick: move |_| {
                active.set(!active());
            },
            h2 { style: "font-size: 1.3rem; margin: 0; text-align: center; line-height: 2.5rem;",
                "{ i18n_lookup(\"options\")?}"
            }
            svg {
                class: if active() { "arrow active" } else { "arrow" },
                width: "20",
                height: "20",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                polyline { points: "6,9 12,15 18,9" }
            }
        }
    }
}
