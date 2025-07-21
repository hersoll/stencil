use dioxus::prelude::*;

use crate::TOOLTIP;

#[component]
pub fn ToolTip(content: String) -> Element {
    rsx! {
        div {
            class: "tooltip_container",
            onmouseenter: move |_| {
                TOOLTIP.write().content = content.clone();
                TOOLTIP.write().visible = true;
            },
            onmouseleave: move |_| {
                TOOLTIP.write().visible = false;
            },
            div { class: "tooltip_icon", "?" }
        }
    }
}

#[component]
pub fn ToolTipDisplay() -> Element {
    let tooltip = TOOLTIP();

    rsx! {
        div {
            class: if tooltip.visible { "tooltip_text visible" } else { "tooltip_text" },
            dangerous_inner_html: "{tooltip.content}",
        }
    }
}
