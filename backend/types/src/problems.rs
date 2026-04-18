use math::Number;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub name: String,
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub identifiers: Vec<Number>,
    pub combinations: usize,
}
