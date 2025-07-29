use dioxus::prelude::*;

use crate::desktop::router::Route;

const DESKTOP_CSS: Asset = asset!("/assets/styling/desktop.css");

#[component]
pub fn DesktopApp() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: DESKTOP_CSS }

        Router::<Route> {}
    }
}
