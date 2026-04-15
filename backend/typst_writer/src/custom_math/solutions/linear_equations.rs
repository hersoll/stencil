use crate::formatting::{SolutionWithSteps, divide_number, subtract_number};
use math::{self, utils::gcd};

/// Equations of the form 3x + 5 = 14. Zeroes for coefficient or constant is not allowed.
pub fn integer_answer(coefficient: i32, unknown: char, constant: i32, final_answer: i32) -> String {
    if coefficient == 0 || constant == 0 {
        panic!("coefficient or constant is 0");
    }

    let mut sol = SolutionWithSteps::new();
    sol.add_aligned(
        format!("{coefficient}{unknown}{constant:+}"),
        coefficient * final_answer + constant,
    )
    .with_step(subtract_number(constant))
    .add_aligned(
        format!("{coefficient}{unknown}"),
        coefficient * final_answer,
    )
    .with_step(divide_number(coefficient))
    .add_aligned(unknown, final_answer);
    sol.to_string()
}

/// Equations of the form 3x + 4 = 14. Zeroes for coefficient or constant is not allowed.
/// Both numerator and denominator must be positive.
/// Ensure that the coefficient is the same as the denominator.
pub fn positive_rational_answer(
    coefficient: i32,
    unknown: char,
    constant: i32,
    numerator: i32,
    denominator: i32,
) -> String {
    if coefficient == 0 || constant == 0 {
        panic!("coefficient or constant is 0");
    }
    if coefficient != denominator {
        panic!("coefficient != denominator")
    }

    let (simplified_numerator, simplified_denominator) =
        math::utils::simplified_fraction(numerator, denominator);
    let gcd = gcd(numerator, denominator);
    let answer_with_simplification = if gcd == denominator {
        format!("{}", numerator / denominator)
    } else if (simplified_numerator, simplified_denominator) != (numerator, denominator) {
        format!(
            r#"({numerator}_(colored(div {gcd})))/({denominator}_(colored(div {gcd}))) = {simplified_numerator} / {simplified_denominator}"#
        )
    } else {
        format!("{numerator}/{denominator}")
    };

    let mut sol = SolutionWithSteps::new();
    sol.add_aligned(
        format!("{coefficient}{unknown}{constant:+}"),
        numerator + constant,
    )
    .with_step(subtract_number(constant))
    .add_aligned(format!("{coefficient}{unknown}"), numerator)
    .with_step(divide_number(coefficient))
    .add_aligned(unknown, answer_with_simplification);
    sol.to_string()
}
