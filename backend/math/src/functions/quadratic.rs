use tracing::error;

use crate::{Number, Polynomial, Term, ZERO, symbols::Symbol};

#[derive(Debug, PartialEq, Eq)]
pub struct QuadraticFunction {
    /// All constructors assert a != 0
    pub a: Number,
    pub b: Number,
    pub c: Number,
}

impl QuadraticFunction {
    pub(crate) fn from_abc(
        a: impl Into<Number>,
        b: impl Into<Number>,
        c: impl Into<Number>,
    ) -> Option<Self> {
        let a = a.into();
        if a == 0 {
            None
        } else {
            Some(QuadraticFunction {
                a,
                b: b.into(),
                c: c.into(),
            })
        }
    }

    pub(crate) fn from_symmetry_distance_k(
        symmetry: impl Into<Number>,
        distance: impl Into<Number>,
        k: impl Into<Number>,
    ) -> Option<Self> {
        let k = k.into();
        if k == 0 {
            return None;
        }
        let symmetry = symmetry.into();
        let distance = distance.into();

        let x1 = symmetry + distance;
        let x2 = symmetry - distance;
        // f(x) = k(x - A)(x - B) = kx^2 + k(-A-B)x + ABk
        let a = k;
        let b = -k * (x1 + x2);
        let c = k * x1 * x2;
        Some(Self { a, b, c })
    }

    pub(crate) fn from_symmetry_distance(
        symmetry: impl Into<Number>,
        distance: impl Into<Number>,
    ) -> Option<Self> {
        Self::from_symmetry_distance_k(symmetry, distance, 1)
    }

    /// The three points must have distinct x-coordinates.
    ///
    /// If the points are on a line, or x-coordinates align, returns None
    pub(crate) fn from_points(
        (x1, y1): (impl Into<Number>, impl Into<Number>),
        (x2, y2): (impl Into<Number>, impl Into<Number>),
        (x3, y3): (impl Into<Number>, impl Into<Number>),
    ) -> Option<Self> {
        let x1 = x1.into();
        let x2 = x2.into();
        let x3 = x3.into();
        let d = (x1 - x2) * (x1 - x3) * (x2 - x3);
        if d == 0 {
            error!(
                "Called QuadraticFunction::from_points() with similar x-values: {x1}, {x2}, {x3}"
            );
            return None;
        }
        let y1 = y1.into();
        let y2 = y2.into();
        let y3 = y3.into();

        let a = (x3 * (y2 - y1) + x2 * (y1 - y3) + x1 * (y3 - y2)) / d;
        let b = (x3.pow(2) * (y1 - y2) + x2.pow(2) * (y3 - y1) + x1.pow(2) * (y2 - y3)) / d;
        let c =
            (x2 * x3 * (x2 - x3) * y1 + x3 * x1 * (x3 - x1) * y2 + x1 * x2 * (x1 - x2) * y3) / d;

        if a == 0 {
            None
        } else {
            Some(QuadraticFunction { a, b, c })
        }
    }

    pub(crate) fn get_x(&self, y: &Number) -> Vec<Number> {
        let p = self.b / self.a;
        let q = (self.c - y) / self.a;

        let symmetry = -p / 2;
        let discr = (p / 2).pow(2) - q;

        use std::cmp::Ordering::*;
        match discr.cmp(&ZERO) {
            Less => vec![],
            Equal => vec![symmetry],
            Greater => vec![
                (symmetry - discr.sqrt()).simplify(),
                (symmetry + discr.sqrt()).simplify(),
            ],
        }
    }

    pub(crate) fn get_y(&self, x: &Number) -> Number {
        self.a * x.pow(2) + self.b * x + self.c
    }

    pub(crate) fn as_poly(&self, variable: &'static Symbol) -> Polynomial {
        (self.a * variable * variable)
            .and(&(self.b * variable))
            .and(&Term::from_num(self.c))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod abc_constructor {
        use super::*;
        #[test]
        fn returns_none_with_a_zero() {
            let quad = QuadraticFunction::from_abc(0, 1, 2);
            assert!(quad.is_none());
        }
    }

    mod symmetry_constructors {
        use super::*;

        #[test]
        fn default() {
            let quad1 = QuadraticFunction::from_symmetry_distance(4, 2);
            let quad2 = QuadraticFunction::from_abc(1, -8, 12);
            assert_eq!(quad1, quad2);
        }

        #[test]
        fn with_symmetry_zero() {
            let quad1 = QuadraticFunction::from_symmetry_distance(0, 2);
            let quad2 = QuadraticFunction::from_abc(1, 0, -4);
            assert_eq!(quad1, quad2);
        }

        #[test]
        fn with_distance_zero() {
            let quad1 = QuadraticFunction::from_symmetry_distance(2, 0);
            let quad2 = QuadraticFunction::from_abc(1, -4, 4);
            assert_eq!(quad1, quad2);
        }

        #[test]
        fn with_k() {
            let quad1 = QuadraticFunction::from_symmetry_distance_k(1.5, 0.5, -2);
            let quad2 = QuadraticFunction::from_abc(-2, 6, -4);
            assert_eq!(quad1, quad2);
        }

        #[test]
        fn returns_none_with_k_zero() {
            let quad = QuadraticFunction::from_symmetry_distance_k(4, 2, 0);
            assert!(quad.is_none());
        }
    }

    mod points_constructor {
        use super::*;

        #[test]
        fn default() {
            let quad1 = QuadraticFunction::from_points((0, 0), (2, 4), (-2, 4));
            let quad2 = QuadraticFunction::from_abc(1, 0, 0);
            assert_eq!(quad1, quad2);
        }

        #[test]
        fn allows_non_integer_results() {
            let quad1 = QuadraticFunction::from_points((0, 0), (4, 4), (-2, 6));
            let a = Number::from((2, 3));
            let b = Number::from((-5, 3));
            let quad2 = QuadraticFunction::from_abc(a, b, 0);
            assert_eq!(quad1, quad2);
        }

        #[test]
        fn allows_non_integer_points() {
            let quad1 = QuadraticFunction::from_points((0, -1), (-0.5, 0), (0.5, 0));
            let quad2 = QuadraticFunction::from_abc(4, 0, -1);
            assert_eq!(quad1, quad2);
        }
    }

    mod methods {
        use super::*;

        #[test]
        fn get_x() {
            let quad = QuadraticFunction::from_symmetry_distance(1, 2).unwrap();
            assert_eq!(quad.get_x(&Number::Integer(-4)), vec![1]);
            assert_eq!(quad.get_x(&Number::Integer(0)), vec![-1, 3]);
            assert_eq!(quad.get_x(&Number::Integer(5)), vec![-2, 4]);
            assert!(quad.get_x(&Number::Integer(-5)).is_empty());
        }

        #[test]
        fn get_y() {
            let quad = QuadraticFunction::from_symmetry_distance(1, 2).unwrap();
            assert_eq!(quad.get_y(&Number::Integer(0)), -3);
            assert_eq!(quad.get_y(&Number::Integer(3)), 0);
            assert_eq!(quad.get_y(&Number::decimal_from_f64(0.5, 1)), -3.75);
        }
    }

    mod printing {
        use super::*;
        use crate::{Evaluable, functions::Function, symbols::X};

        #[test]
        fn replacements_default() {
            let func = Function::quadratic_from_abc(5, 3, 4);
            let replacements = [(X, &Number::Integer(2))];
            assert_eq!(
                func.print_replacements(&replacements),
                "y = 5 dot colored(2)^2+3 dot colored(2)+4"
            );
        }

        #[test]
        fn replacements_negative() {
            let func = Function::quadratic_from_abc(5, 3, 4);
            let replacements = [(X, &Number::Integer(-2))];
            assert_eq!(
                func.print_replacements(&replacements),
                "y = 5 dot colored((-2))^2+3 dot colored((-2))+4"
            );
        }

        #[test]
        fn replacements_ones() {
            let func = Function::quadratic_from_abc(1, 1, 1);
            let replacements = [(X, &Number::Integer(2))];
            assert_eq!(
                func.print_replacements(&replacements),
                "y = colored(2)^2+colored(2)+1"
            );
        }

        #[test]
        fn replacements_negative_ones() {
            let func = Function::quadratic_from_abc(-1, -1, -1);
            let replacements = [(X, &Number::Integer(2))];
            assert_eq!(
                func.print_replacements(&replacements),
                "y = -colored(2)^2-colored(2)-1"
            );
        }

        #[test]
        fn replacements_zeroes() {
            let func = Function::quadratic_from_abc(2, 0, 0);
            let replacements = [(X, &Number::Integer(3))];
            assert_eq!(
                func.print_replacements(&replacements),
                "y = 2 dot colored(3)^2"
            );
        }

        #[test]
        fn evaluation_by_parts_default() {
            let func = Function::quadratic_from_abc(2, 3, 4);
            let replacements = [(X, &Number::Integer(5))];
            assert_eq!(
                func.print_evaluation_by_parts(&replacements),
                "y = 2 dot 25+3 dot 5+4"
            );
        }
    }
}
