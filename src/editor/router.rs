use dioxus::prelude::*;

use crate::editor::chapter_editor::ChapterEditor;
use crate::editor::course_editor::CourseEditor;
use crate::editor::landing_page::LandingPage;
use crate::editor::navbar::NavBar;
use crate::editor::problem_editor::ProblemEditor;
use crate::editor::topic_editor::TopicEditor;

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
