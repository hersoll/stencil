use dioxus::prelude::*;

use crate::editor::chapter_page::ChapterPage;
use crate::editor::course_page::CoursePage;
use crate::editor::landing_page::LandingPage;
use crate::editor::navbar::NavBar;
use crate::editor::problem_page::ProblemPage;
use crate::editor::topic_page::TopicPage;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(NavBar)]
    #[route("/")]
    LandingPage {},
    #[route("/course")]
    CoursePage {},
    #[route("/chapter")]
    ChapterPage {},
    #[route("/topic")]
    TopicPage {},
    #[route("/problem")]
    ProblemPage {},
}
