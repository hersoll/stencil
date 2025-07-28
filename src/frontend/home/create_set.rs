use crate::frontend::i18n_lookup;
use dioxus::prelude::*;

use crate::frontend::Sets;
use crate::shared::ProblemSetData;

#[component]
pub fn CreateSet(
    set_data: Signal<ProblemSetData>,
    sets: Signal<Sets>,
    set_pusher: EventHandler<()>,
) -> Element {
    // True if you try to add set without adding a problem type
    let mut creation_error = use_signal(|| false);
    use_effect(move || {
        if !set_data().topics.is_empty() {
            creation_error.set(false);
        }
    });
    rsx! {
        div { class: "create_set_button_container",
            button {
                class: "button",
                onclick: move |_| {
                    if set_data().topics.is_empty() {
                        creation_error.set(true);
                    } else {
                        creation_error.set(false);
                        set_pusher.call(());
                    }
                },
                "{i18n_lookup(\"create_set\")?}"
            }
        }

        div { class: "create_set_error",
            if creation_error() {
                "{i18n_lookup(\"create_set_error\")?}"
            } else {
                ""
            }
        }
    }
}
