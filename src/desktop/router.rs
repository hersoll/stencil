use dioxus::prelude::*;

use crate::desktop::chapter_editor::ChapterEditor;
use crate::desktop::course_editor::CourseEditor;
use crate::desktop::landing_page::LandingPage;
use crate::desktop::navbar::NavBar;
use crate::desktop::problem_editor::ProblemEditor;
use crate::desktop::topic_editor::TopicEditor;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    LandingPage {},
    #[route("/course")]
    CourseEditor {},
    #[route("/chapter")]
    ChapterEditor {},
    #[route("/topic")]
    TopicEditor {},
    #[route("/problem")]
    ProblemEditor {},
}
