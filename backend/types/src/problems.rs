#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub name: String,
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub identifiers: Vec<i32>,
    pub combinations: usize,
}
