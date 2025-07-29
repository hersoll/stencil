use dioxus::prelude::*;

use crate::desktop::course_editor::CourseEditor;
use crate::desktop::landing_page::LandingPage;
use crate::desktop::navbar::NavBar;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    LandingPage {},
    #[route("/course")]
    CourseEditor {},
}
