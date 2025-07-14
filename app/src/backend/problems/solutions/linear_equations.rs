use crate::backend::{math_utils::gcd, problems, typst_formatting};

/// Equations of the form 3x + 5 = 14. Zeroes for coefficient or constant is not allowed.
pub fn integer_answer(coefficient: i32, unknown: char, constant: i32, final_answer: i32) -> String {
    if coefficient == 0 || constant == 0 {
        panic!("coefficient or constant is 0");
    }

    let solution = format!(
        "{cf}{unknown} {co:+} &= {rhs} \\ {m_co:+} \\
                {cf}{unknown} &= {cf_a} \\ div {cf_par} \\
                    {unknown} &= {a} \\",
        cf = coefficient,
        cf_par = if coefficient < 0 {
            format!("({coefficient})")
        } else {
            format!("{coefficient}")
        },
        co = constant,
        rhs = coefficient * final_answer + constant,
        m_co = -constant,
        cf_a = coefficient * final_answer,
        a = final_answer
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
    let answer_with_simplification = if (simplified_numerator, simplified_denominator)
        != (numerator, denominator)
    {
        format!(
            r#"({numerator} gray(div {gcd}))/({denominator}gray(div {gcd})) = {simplified_numerator} / {simplified_denominator}"#
        )
    } else {
        format!("{numerator}/{denominator}")
    };

    let solution = format!(
        "{cf}{unknown} {constant:+} &= {rhs} \\ {m_co:+} \\
                {cf}{unknown} &= {numerator} \\ div {cf} \\
                    {unknown} &= {answer_with_simplification} \\",
        cf = coefficient,
        rhs = numerator + constant,
        m_co = -constant,
    );
    typst_formatting::equation_solution(solution)
}
