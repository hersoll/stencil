use anyhow::Result;
use macros::problem;
use math::{MathDisplay, num_gen};
use registry::get_solution;
use types::{lang::Language, problems::Problem};

/// 3 + 4 * 2
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn addition_multiplication(id: i32, lang: Language) -> Result<Problem> {
    let (term_1, t1_range) = num_gen::integer().range(2, 10).and_random();
    let (factor_1, f1_range) = num_gen::integer().range(1, 10).and_random();
    let (factor_2, f2_range) = num_gen::integer().range(2, 10).and_random();
    let product = factor_1 * factor_2;

    let question = format!("${term_1} + {factor_1} dot {factor_2}$");
    let answer = term_1 + product;
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "{solution_text} \\
        $ {term_1} + colored({factor_1} dot {factor_2}) = {term_1} + colored({product}) = {answer} $
        "
    );
    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![term_1, factor_1, factor_2],
        combinations: t1_range.len() + f1_range.len() + f2_range.len(),
    })
}

/// 10 - 4 * 2
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn subtraction_multiplication(id: i32, lang: Language) -> Result<Problem> {
    let (factor_1, f1_range) = num_gen::integer().range(1, 5).and_random();
    let (factor_2, f2_range) = num_gen::integer().range(2, 5).and_random();
    let product = factor_1 * factor_2;
    // If we want a positive answer, the term must be larger than the product
    let term_1 = product + num_gen::integer().range(1, 5).random();

    let question = format!("${term_1} - {factor_1} dot {factor_2}$");
    let answer = term_1 - product;
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "{solution_text} \\
        $ {term_1} - colored({factor_1} dot {factor_2}) = {term_1} - colored({product}) = {answer} $
        "
    );
    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![term_1, factor_1, factor_2],
        combinations: f1_range.len() + f2_range.len(),
    })
}

/// 5 * 3 + 4 * 2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn mult_add_mult(id: i32, lang: Language) -> Result<Problem> {
    let (factor_1, f_range) = num_gen::integer().range(2, 10).and_random();
    let factor_2 = f_range.random();
    let factor_3 = f_range.random();
    let factor_4 = f_range.random();
    let product_1 = factor_1 * factor_2;
    let product_2 = factor_3 * factor_4;

    let question = format!("${factor_1} dot {factor_2} + {factor_3} dot {factor_4}$");
    let answer = product_1 + product_2;
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "{solution_text} \\
        $ colored({factor_1} dot {factor_2}) + colored({factor_3} dot {factor_4}) =
        colored({product_1}) + colored({product_2}) = {answer} $"
    );
    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![factor_1, factor_2, factor_3, factor_4],
        combinations: f_range.len().pow(4),
    })
}

/// 5 * 7 - 4 * 2
/// Absolute difficulty: 1
/// Relative difficulty: 2
#[problem]
fn mult_sub_mult(id: i32, lang: Language) -> Result<Problem> {
    let (factor_1, f_range) = num_gen::integer().range(3, 10).and_random();
    let factor_2 = f_range.random();
    // To both make the answer positive, and to make the subtraction "look" correct
    // (i.e.) 7 - 4 in the example is a positive number, we need factor_3 to be smaller than factor_2
    let factor_3 = num_gen::integer().range(2, factor_2).random();
    // If we also make factor_4 <= factor_1, we assert a non-negative difference in the end
    let factor_4 = num_gen::integer().range(2, factor_1).random();
    let product_1 = factor_1 * factor_2;
    let product_2 = factor_3 * factor_4;

    let question = format!("${factor_1} dot {factor_2} - {factor_3} dot {factor_4}$");
    let answer = product_1 - product_2;
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "{solution_text} \\
        $ colored({factor_1} dot {factor_2}) - colored({factor_3} dot {factor_4}) =
        colored({product_1}) - colored({product_2}) = {answer} $"
    );
    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![factor_1, factor_2, factor_3, factor_4],
        combinations: f_range.len().pow(2),
    })
}

/// 10 + (2 + 1) * 3
/// Absolute difficulty: 1
/// Relative difficulty: 3
#[problem]
fn add_par_mult(id: i32, lang: Language) -> Result<Problem> {
    let (term_1, t_range) = num_gen::integer().range(1, 5).and_random();
    // The numbers inside the parentheses
    let term_2 = t_range.random();
    let term_3 = t_range.random();
    let (factor, f_range) = num_gen::integer().range(2, 10).and_random();

    let par_sum = term_2 + term_3;
    let product = par_sum * factor;
    let answer = term_1 + product;

    let question = format!("${term_1} + ({term_2} + {term_3}) dot {factor}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "{solution_text} \\
        $ {term_1} + colored(({term_2} + {term_3})) dot {factor} = {term_1} + colored({par_sum} dot {factor}) = \\
        = {term_1} + {product} = {answer} $"
    );
    Ok(Problem {
        id,
        question,
        answer: answer.as_math(),
        solution,
        identifiers: vec![term_1, term_2, term_3, factor],
        combinations: f_range.len() * t_range.len().pow(3),
    })
}
