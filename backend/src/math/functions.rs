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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::typst_utils::graphing::Graph;

    #[test]
    fn linear_function_integers_get_y() {
        let f = Graph::linear(1, 2);
        let g = Graph::linear(0, 2);
        let h = Graph::linear(-3, -5);
        let x = Number::Integer(-1);
        assert_eq!(f.function.get_y(&x), Number::Integer(1));
        assert_eq!(g.function.get_y(&x), Number::Integer(2));
        assert_eq!(h.function.get_y(&x), Number::Integer(-2));
    }
}
