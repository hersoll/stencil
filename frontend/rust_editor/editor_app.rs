use dioxus::prelude::*;

use crate::editor::router::Route;

const EDITOR_CSS: Asset = asset!("/assets/styling/editor.css");

#[component]
pub fn EditorApp() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: EDITOR_CSS }

        Router::<Route> {}
    }
}
