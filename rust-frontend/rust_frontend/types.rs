use dioxus::signals::Signal;

pub type Sets = Vec<Signal<crate::shared::ProblemSetData>>;

#[derive(Debug, Clone)]
pub struct TooltipData {
    pub content: String,
    pub visible: bool,
}
