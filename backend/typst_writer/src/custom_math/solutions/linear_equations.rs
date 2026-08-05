use crate::formatting::{divide_number, subtract_number};
use math::{self, Number, symbols::Symbol, utils::gcd};
use types::problems::Solution;

/// Equations of the form 3x + 5 = 14. Zeroes for coefficient or constant is not allowed.
pub fn integer_answer(coefficient: i32, unknown: char, constant: i32, final_answer: i32) -> String {
    if coefficient == 0 || constant == 0 {
        panic!("coefficient or constant is 0");
    }

    let mut sol = Solution::with_steps();
    sol.aligned(
        format!("{coefficient}{unknown}{constant:+}"),
        coefficient * final_answer + constant,
    )
    .step(subtract_number(constant))
    .aligned(
        format!("{coefficient}{unknown}"),
        coefficient * final_answer,
    )
    .step(divide_number(coefficient))
    .aligned(unknown, final_answer);
    sol.to_string()
}

/// Equations of the form 3x + 4 = 14. Zeroes for coefficient or constant is not allowed.
/// Both numerator and denominator must be positive.
/// Ensure that the coefficient is the same as the denominator.
pub fn positive_rational_answer(
    coefficient: Number,
    unknown: &Symbol,
    constant: Number,
    numerator: Number,
    denominator: Number,
) -> String {
    if coefficient == 0 || constant == 0 {
        panic!("coefficient or constant is 0");
    }
    if coefficient != denominator {
        panic!("coefficient != denominator")
    }

    // This is a mess. I'm sorry. I'm tired. 26-04-19

    let answer_with_simplification = if let (Number::Integer(num), Number::Integer(denom)) =
        (numerator, denominator)
    {
        let (simplified_numerator, simplified_denominator) =
            math::utils::simplified_fraction(num, denom);
        let gcd = gcd(numerator, denominator);
        if gcd == denominator {
            format!("{}", numerator / denominator)
        } else if (simplified_numerator, simplified_denominator) != (num, denom) {
            format!(
                r#"({numerator}_(colored(div {gcd})))/({denominator}_(colored(div {gcd}))) = {simplified_numerator} / {simplified_denominator}"#
            )
        } else {
            format!("{numerator}/{denominator}")
        }
    } else {
        String::new()
    };

    let mut sol = Solution::with_steps();
    sol.aligned(
        format!("{coefficient}{unknown}{constant:+}"),
        numerator + constant,
    )
    .step(subtract_number(constant))
    .aligned(format!("{coefficient}{unknown}"), numerator)
    .step(divide_number(coefficient))
    .aligned(unknown, answer_with_simplification);
    sol.to_string()
}
