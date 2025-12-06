use crate::{math_utils::gcd, problems, typst_formatting};

/// Equations of the form 3x + 5 = 14. Zeroes for coefficient or constant is not allowed.
pub fn integer_answer(coefficient: i32, unknown: char, constant: i32, final_answer: i32) -> String {
    if coefficient == 0 || constant == 0 {
        panic!("coefficient or constant is 0");
    }

    let solution = format!(
        "{coefficient}{unknown} {constant:+} &= {rhs} \\ {sub_constant} \\
                {coefficient}{unknown} &= {cf_a} \\ div {cf_par} \\
                    {unknown} &= {final_answer} \\",
        cf_par = typst_formatting::parentheses(coefficient),
        rhs = coefficient * final_answer + constant,
        sub_constant = typst_formatting::subtract_number(constant),
        cf_a = coefficient * final_answer,
    );
    typst_formatting::equation_solution(solution)
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
        problems::math_utils::simplified_fraction(numerator, denominator);
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

    let solution = format!(
        "{coefficient}{unknown} {constant:+} &= {rhs} \\ {sub_constant} \\
                {coefficient}{unknown} &= {numerator} \\ div {coefficient} \\
                    {unknown} &= {answer_with_simplification} \\",
        rhs = numerator + constant,
        sub_constant = typst_formatting::subtract_number(constant),
    );
    typst_formatting::equation_solution(solution)
}
