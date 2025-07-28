use crate::frontend::Home;
use dioxus::prelude::*;

#[derive(Routable, Clone)]
pub enum Route {
    #[route("/")]
    Home {},
}
