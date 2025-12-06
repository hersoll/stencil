use dioxus::prelude::*;

use crate::frontend::{Route, i18n_lookup};

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        div { class: "page_not_found_container",
            p { class: "page_not_found_heading", "{i18n_lookup(\"page_not_found_heading\")?}" }
            p { class: "page_not_found_which",
                "{i18n_lookup(\"page_not_found_which\")?} {route.join(\"/\")}"
            }
            Link { to: Route::Home {}, class: "page_not_found_text",
                "{i18n_lookup(\"page_not_found_text\")?}"
            }
        }
    }
}
