use crate::frontend::Home;
use crate::frontend::PageNotFound;
use dioxus::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}
