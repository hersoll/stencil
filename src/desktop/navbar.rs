use dioxus::prelude::*;

use crate::desktop::router::Route;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav { "This is the navbar" }
        Outlet::<Route> {}
    }
}
