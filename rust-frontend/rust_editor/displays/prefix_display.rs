use dioxus::prelude::*;

use crate::shared::PrefixData;

#[component]
pub fn PrefixDisplay(
    selected_prefix: Signal<Option<PrefixData>>,
    prefixes: Signal<Vec<PrefixData>>,
) -> Element {
    rsx! {
        div {
            class: "available_element prefix header",
            style: "position: relative;",
            p { "Name" }
            p { "Text" }
            p { "Group Text" }
        }
        for prefix in prefixes() {
            div {
                class: "available_element prefix item",
                style: if let Some(selected) = selected_prefix() { if selected.id == prefix.id { "background-color: gray;" } else { "" } },
                onclick: move |_| selected_prefix.set(Some(prefix.clone())),
                p { "{prefix.name}" }
                p { "{prefix.text_sv}" }
                p { "{prefix.group_text_sv}" }
            }
        }
    }
}
