use math::Number;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Problem {
    pub id: i32,
    pub question: String,
    pub answer: String,
    pub solution: String,
    pub identifiers: Vec<Number>,
    pub combinations: usize,
}
