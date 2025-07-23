use crate::frontend::components::toggle::Toggle;
use dioxus::prelude::*;

use crate::frontend::i18n_lookup;
use crate::shared::DocumentOptions;

#[component]
pub fn DocumentOptionsColor(options: Signal<DocumentOptions>) -> Element {
    let mut color = use_signal(|| true);
    use_effect(move || {
        color.set(options().color);
    });

    let change_color = move |new_value| options.write().color = new_value;

    rsx! {
        div {
            p { "{i18n_lookup(\"document_option_color\")?}:" }
            Toggle { active: color, toggle_closure: change_color }
        }
    }
}
