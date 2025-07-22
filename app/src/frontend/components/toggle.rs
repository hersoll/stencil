use dioxus::prelude::*;

/// Note that the toggle_closure should have a bool as a parameter, not an event.
#[component]
pub fn Toggle(active: Signal<bool>, toggle_closure: Option<EventHandler<bool>>) -> Element {
    rsx! {
        label { class: "toggle",
            input {
                r#type: "checkbox",
                checked: active(),
                onchange: move |evt| {
                    active.set(evt.checked());
                    if let Some(callback) = &toggle_closure {
                        callback.call(evt.checked());
                    }
                },
            }
            span { class: "slider" }
        }
    }
}
