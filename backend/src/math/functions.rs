use crate::math::{Number, ZERO};

/// The Function enum contains information about which kind of function it is (duh), but also numbers
/// that are specific to that kind of plot. This makes ergonomics easier when matching over the
/// kinds since you can do Function::Linear(k, m) => ... and then use k and m by those names.
pub enum Function {
    /// k, m
    Linear(Number, Number),
    /// start, change
    Exponential(Number, Number),
}

impl Function {
    pub fn get_x(&self, y: &Number) -> Option<Number> {
        match self {
            Function::Linear(k, m) => Some((y - m) / k),
            Function::Exponential(c, a) => {
                // No solution if y and c have opposite signs
                if y * c < ZERO {
                    None
                } else {
                    // y = c a^x => x = lg(y/c) / lg(a)
                    Some(((y / &c).value().log2() / a.value().log2()).into())
                }
            }
        }
    }

    pub fn get_y(&self, x: &Number) -> Number {
        match self {
            Function::Linear(k, m) => k * x + m,
            Function::Exponential(c, a) => c * a.value().powf(x.value()),
        }
    }
}
