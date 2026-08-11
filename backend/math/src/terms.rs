mod operations;

use crate::num_gen::NumberGenerator;
use crate::symbols::Symbol;
use crate::utils::parenthesize;
use crate::{Evaluable, HasCoef, Polynomial, Replacement, Replacements};
use crate::{Number, PolynomialVariable, VariableList, num_gen};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Term {
    pub coefficient: Number,
    pub variables: VariableList,
    pub colored: bool,
}

impl Term {
    /// Constructor to explicitly create a [`Term`] from [`Symbols`](Symbol).
    ///
    /// Note that if you have more than a `Symbol`, you can do `3 * X` or `X * X`.
    /// (You can also do `1 * X` instead of calling this function, but that looks a bit odd)
    pub fn from_var<T: Into<VariableList>>(var: T) -> Self {
        let var = var.into();
        Self {
            coefficient: Number::Integer(1),
            variables: var,
            colored: false,
        }
    }

    /// Create a [`Term`] from a single [`Number`], or a primitive that can be converted to a `Number`.
    pub fn from_num<T: Into<Number>>(num: T) -> Self {
        let num = num.into();
        Self {
            coefficient: num,
            variables: VariableList::empty(),
            colored: false,
        }
    }

    /// Ergonomic constructor when the [`Number`] and [`Symbol`] are tedious to construct manually.
    ///
    /// For example: `Term::from_num_and_vars((4, 5), (X, 5))`
    pub fn from_num_and_vars<T: Into<Number>, U: Into<VariableList>>(num: T, vars: U) -> Self {
        let num = num.into();
        let vars = vars.into();
        Self {
            coefficient: num,
            variables: vars,
            colored: false,
        }
    }

    /// Alias method to quickly create a `Polynomial`.
    ///
    /// Example:
    /// ```rust
    /// use math::Term;
    /// use math::symbols::X;
    /// let k_term = 3 * X;
    /// let m_term = Term::from_num(-2);
    /// let function = k_term.and(&m_term);
    /// assert_eq!(function.to_string(), String::from("3x-2"));
    /// ```
    pub fn and<T>(&self, other: &T) -> Polynomial
    where
        T: Into<Term> + Clone,
    {
        let other = other.clone().into();
        Polynomial::from_terms(&[self, &other])
    }

    /// Returns the Term with the coefficient changed to its absolute value.
    pub fn abs(&self) -> Self {
        Self {
            coefficient: self.coefficient.abs(),
            variables: self.variables.clone(),
            colored: self.colored,
        }
    }

    /// Helper function. Makes sure one of the [`Terms`](Term) are positive.
    ///
    /// Makes one positive if not.
    pub fn assert_one_positive(term1: &mut Term, term2: &mut Term) {
        if *term1 < 0 && *term2 < 0 {
            let random = num_gen::integer().range(0, 1).random();
            if random == 0 {
                *term1 = -term1.clone();
            } else {
                *term2 = -term2.clone();
            }
        }
    }

    /// Returns whether the term is a constant term or not
    fn is_constant(&self) -> bool {
        self.variables.list.is_empty() || self.variables.list.iter().all(|var| var.exponent == 0)
    }
}
impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.colored && self.coefficient != 0 {
            write!(f, " colored(")?;
        }
        if f.sign_plus() && self.coefficient > 0 {
            write!(f, "+")?;
        }

        if self.coefficient == 1 {
            if self.is_constant() {
                write!(f, "1")?;
            } else {
                write!(f, "{}", self.variables)?;
            }
        } else if self.coefficient == -1 {
            if self.is_constant() {
                write!(f, "-1")?;
            } else {
                write!(f, "-{}", self.variables)?;
            }
        } else if self.coefficient == 0 && f.sign_plus() {
            write!(f, "")?;
        } else {
            match self.coefficient {
                Number::Integer(_) | Number::Decimal { .. } => {
                    write!(f, "{}{}", self.coefficient, self.variables)?
                }
                Number::Fraction {
                    numerator,
                    denominator,
                } => write!(
                    f,
                    "{sign}({}{})/{}",
                    if numerator.abs() != 1 || self.variables.list.is_empty() {
                        numerator.abs().to_string()
                    } else {
                        String::new()
                    },
                    self.variables,
                    denominator.abs(),
                    sign = if self.coefficient.value() < 0.0 {
                        "-"
                    } else {
                        ""
                    }
                )?,
                Number::Irrational { symbol, .. } => write!(f, "{symbol} {}", self.variables)?,
            };
        }
        if self.colored && self.coefficient != 0 {
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl Evaluable for Term {
    fn print_replacements(&self, replacements: &[Replacement]) -> String {
        let replacements = Replacements::from_array(replacements);

        use std::fmt::Write;
        // Always print the coefficient first
        let mut s = String::new();
        let coefficient_written = self.coefficient.abs() != 1;
        if self.coefficient == -1 && !self.variables.list.is_empty() {
            write!(s, "-").unwrap();
        } else if self.coefficient != 1 {
            write!(s, "{}", self.coefficient).unwrap();
        } else if self.coefficient == 1 && self.variables.list.is_empty() {
            write!(s, "1").unwrap()
        } else if self.coefficient == -1 && self.variables.list.is_empty() {
            write!(s, "-1").unwrap()
        }

        // For each variable, replace that variable with a number (if it exists in Replacements)
        for (i, var) in self.variables.list.iter().enumerate() {
            match replacements
                .0
                .iter()
                .find(|(symbol, _)| *symbol == var.symbol)
            {
                None => write!(&mut s, "{var}").unwrap(), // No replacement, just print the var
                Some((_, num)) => {
                    // Adding dot at the ending if/else and then trimming it here might look
                    // weird. This is due to the fact that when we have, say, abcd and want to
                    // replace b with 2, the output should be "a dot 2 dot cd".
                    //
                    // The replacement is what causes the surrounding dots, since symbols don't
                    // have dots between them. That's why we need to add them in the beginning and
                    // the end, but this means we must trim "dot" on successive replacements
                    s = s.trim_end_matches(" dot ").to_string();
                    if coefficient_written {
                        write!(s, " dot ").unwrap()
                    }
                    write!(
                        &mut s,
                        "colored({}){}{}",
                        parenthesize(num),
                        if var.exponent > 1 {
                            format!("^{}", var.exponent)
                        } else {
                            String::new()
                        },
                        if i < self.variables.list.len() - 1 {
                            " dot "
                        } else {
                            ""
                        }
                    )
                    .unwrap()
                }
            }
        }

        s
    }

    // Very similar to print_replacements(), except the exponentiation is calculated
    // instead of written out.
    // Will also panic :)
    fn print_evaluation_by_parts(&self, replacements: &[Replacement]) -> String {
        let replacements = Replacements::from_array(replacements);

        use std::fmt::Write;
        // Always print the coefficient first
        let mut s = self.coefficient.to_string();
        // For each variable, replace that variable with a number
        for var in self.variables.list.iter() {
            match replacements
                .0
                .iter()
                .find(|(symbol, _)| *symbol == var.symbol)
            {
                None => panic!("Didn't replace {} in {self}", var.symbol),
                Some((_, num)) => {
                    // Compared to print_replacements() , this will always print " dot "
                    // since everything must be replaced
                    let evaluated_variable = parenthesize(&num.pow(Number::Integer(var.exponent)));
                    write!(&mut s, " dot {evaluated_variable}",).unwrap()
                }
            }
        }

        s
    }

    fn evaluate(&self, replacements: &[Replacement]) -> Number {
        let replacements = Replacements::from_array(replacements);

        // Term is, say, 3xy^2 => Start the result at 3
        let mut result = self.coefficient;
        // For each variable, substitute the symbol for the number and exponentiate it
        self.variables.list.iter().for_each(|v| {
            match replacements.0.iter().find(|(symbol, _)| *symbol == v.symbol) {
                Some((_, num)) => result *= Number::from(num.value().powf(v.exponent as f64)),
                None => panic!("Variable {v} not in replacements {replacements:#?}. (Panic should not be reached if called from polynomial.evaluate())"),
            }
        });
        result
    }
}

impl From<Number> for Term {
    fn from(value: Number) -> Self {
        Self {
            coefficient: value,
            variables: VariableList::empty(),
            colored: false,
        }
    }
}

impl From<i32> for Term {
    fn from(value: i32) -> Self {
        Self {
            coefficient: value.into(),
            variables: VariableList::empty(),
            colored: false,
        }
    }
}

impl From<(i32, i32)> for Term {
    fn from(value: (i32, i32)) -> Self {
        Self {
            coefficient: value.into(),
            variables: VariableList::empty(),
            colored: false,
        }
    }
}

impl From<f64> for Term {
    fn from(value: f64) -> Self {
        Self {
            coefficient: value.into(),
            variables: VariableList::empty(),
            colored: false,
        }
    }
}

impl From<(&'static Symbol, i32)> for Term {
    fn from(value: (&'static Symbol, i32)) -> Self {
        Self {
            coefficient: 1.into(),
            variables: VariableList::from((value.0, value.1)),
            colored: false,
        }
    }
}

impl From<&'static Symbol> for Term {
    fn from(value: &'static Symbol) -> Self {
        Self {
            coefficient: 1.into(),
            variables: VariableList::from(value),
            colored: false,
        }
    }
}

impl<T> From<(T, VariableList)> for Term
where
    T: Into<Number>,
{
    fn from(value: (T, VariableList)) -> Self {
        Term {
            coefficient: value.0.into(),
            variables: value.1,
            colored: false,
        }
    }
}

impl<T, U> From<(T, U)> for Term
where
    T: Into<Number>,
    U: Into<PolynomialVariable>,
{
    fn from(value: (T, U)) -> Self {
        Self {
            coefficient: value.0.into(),
            variables: VariableList::from(value.1),
            colored: false,
        }
    }
}

impl PartialOrd<Number> for Term {
    fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
        Some(self.coefficient.cmp(other))
    }
}

impl PartialEq<Number> for Term {
    fn eq(&self, other: &Number) -> bool {
        self.coefficient == *other
    }
}

impl PartialOrd<i32> for Term {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.coefficient.partial_cmp(other)
    }
}

impl PartialEq<i32> for Term {
    fn eq(&self, other: &i32) -> bool {
        self.coefficient == *other
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.coefficient.cmp(&other.coefficient)
    }
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.coefficient == other.coefficient
    }
}
impl Eq for Term {}

impl HasCoef for Term {
    fn coef(&self) -> Number {
        self.coefficient
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static X: Symbol = Symbol("x");
    static A: Symbol = Symbol("a");

    #[test]
    fn term_creation() {
        let t1 = 3 * Term::from_var(&X);
        let t2 = Term::from_var((&X, 3));
        let t3 = Term::from_num(6);

        assert_eq!(t1.to_string(), "3x");
        assert_eq!(t2.to_string(), "x^3");
        assert_eq!(t3.to_string(), "6");
    }

    #[test]
    fn term_displays() {
        let t_a = Term::from_var(&A);
        let t_one = Term::from_num(1);
        let t_m_one = Term::from_num(-1);
        let t_zero = Term::from_num(0);
        let mut t_color = -3 * Term::from_var(&X);
        let fractional_term = Number::Fraction {
            numerator: 3,
            denominator: 5,
        } * &X;
        t_color.colored = true;
        assert_eq!(format!("{t_a}"), "a");
        assert_eq!(format!("{t_a:+}"), "+a");
        assert_eq!(format!("{t_one}"), "1");
        assert_eq!(format!("{t_one:+}"), "+1");
        assert_eq!(format!("{t_m_one}"), "-1");
        assert_eq!(format!("{t_m_one:+}"), "-1");
        assert_eq!(format!("{t_zero}"), "0");
        assert_eq!(format!("{t_zero:+}"), "");
        assert_eq!(format!("{t_color}"), " colored(-3x)");
        assert_eq!(format!("{fractional_term}"), "(3x)/5");
    }

    #[test]
    fn checks_constants() {
        let no_vars = Term::from_num(5);
        assert!(no_vars.is_constant());
        let exponent_is_0 = Term::from_var((&X, 0));
        assert!(exponent_is_0.is_constant());
        assert_eq!(format!("{no_vars}{exponent_is_0:+}"), "5+1");
    }

    mod evaluations {
        use crate::symbols::{X, Y};

        use super::*;

        #[test]
        fn evaluates() {
            let t1 = 3 * X * Y;
            let t2 = 3 * X * X * Y * Y * Y;
            let replacements = [(X, &Number::Integer(10)), (Y, &Number::Integer(1))];
            assert_eq!(t1.evaluate(&replacements), 30);
            assert_eq!(t1.evaluate(&replacements).to_string(), "30");
            assert_eq!(t2.evaluate(&replacements), 300);
            assert_eq!(t2.evaluate(&replacements).to_string(), "300");

            let replacements = [(X, &Number::Integer(-2)), (Y, &Number::Integer(4))];
            assert_eq!(t1.evaluate(&replacements), -24);
            assert_eq!(t1.evaluate(&replacements).to_string(), "-24");
            assert_eq!(t2.evaluate(&replacements), 768);
            assert_eq!(t2.evaluate(&replacements).to_string(), "768");

            let replacements = vec![(X, &Number::Integer(100)), (Y, &Number::Integer(0))];
            assert_eq!(t1.evaluate(&replacements), 0);
            assert_eq!(t1.evaluate(&replacements).to_string(), "0");
            assert_eq!(t2.evaluate(&replacements), 0);
            assert_eq!(t2.evaluate(&replacements).to_string(), "0");
        }

        #[should_panic]
        #[test]
        fn evaluate_panics() {
            let term = 3 * X * Y;
            let replacements = [(Y, &Number::Integer(1))];
            assert_eq!(term.evaluate(&replacements), 30); // not everything replaced
        }

        #[test]
        fn prints_replacements_and_evaluations() {
            let term = 2 * X * X * Y;
            let full_replacement = [(X, &Number::Integer(4)), (Y, &Number::Integer(-2))];
            let partial_replacement = [(X, &Number::Integer(4))];
            let zero_replacement = [(X, &Number::Integer(0)), (Y, &Number::Integer(0))];

            assert_eq!(
                term.print_replacements(&full_replacement),
                "2 dot colored(4)^2 dot colored((-2))"
            );
            assert_eq!(
                term.print_evaluation_by_parts(&full_replacement),
                "2 dot 16 dot (-2)"
            );

            assert_eq!(
                term.print_replacements(&partial_replacement),
                "2 dot colored(4)^2 dot y"
            );

            assert_eq!(
                term.print_replacements(&zero_replacement),
                "2 dot colored(0)^2 dot colored(0)"
            );
            assert_eq!(
                term.print_evaluation_by_parts(&zero_replacement),
                "2 dot 0 dot 0"
            );
        }
    }
}
