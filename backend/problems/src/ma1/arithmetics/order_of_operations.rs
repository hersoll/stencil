use anyhow::Result;
use macros::problem;
use math::num_gen::{self, NumberGenerator};
use registry::get_solution;
use types::{
    lang::Language,
    problems::{Problem, ProblemParameters},
};

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
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![term_1, factor_1, factor_2],
        combinations: t1_range.len() + f1_range.len() + f2_range.len(),
    }))
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
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![term_1, factor_1, factor_2],
        combinations: f1_range.len() + f2_range.len(),
    }))
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
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![factor_1, factor_2, factor_3, factor_4],
        combinations: f_range.len().pow(4),
    }))
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
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![factor_1, factor_2, factor_3, factor_4],
        combinations: f_range.len().pow(2),
    }))
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
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![term_1, term_2, term_3, factor],
        combinations: f_range.len() * t_range.len().pow(3),
    }))
}

/// 20 - (8 - 3) * 3
/// Absolute difficulty: 1
/// Relative difficulty: 3
#[problem]
fn sub_par_mult(id: i32, lang: Language) -> Result<Problem> {
    // First of all, the expression in the parenthesis needs to be positive
    let (par_1, p_range) = num_gen::integer().range(2, 10).and_random();
    let par_2 = num_gen::integer().range(1, par_1).random();

    let (factor, f_range) = num_gen::integer().range(2, 5).and_random();

    let par_diff = par_1 - par_2;
    let product = par_diff * factor;
    // For a positive answer, the first term needs to be larger than the product
    let (term_1, t_range) = num_gen::integer().range(product, product + 5).and_random();
    let answer = term_1 - product;

    let question = format!("${term_1} - ({par_1} - {par_2}) dot {factor}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "{solution_text} \\
        $ {term_1} - colored(({par_1} - {par_2})) dot {factor} = {term_1} - colored({par_diff} dot {factor}) = \\
        = {term_1} - {product} = {answer} $"
    );
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![term_1, par_1, factor],
        combinations: t_range.len() * p_range.len() * f_range.len(),
    }))
}

/// 7 + 3^2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn add_power(id: i32, lang: Language) -> Result<Problem> {
    let (term, t_range) = num_gen::integer().range(1, 7).and_random();
    let (base, base_range) = num_gen::integer().range(2, 5).and_random();
    let exp = 2;
    let power = base.pow(exp);

    let answer = term + power;

    let question = format!("${term} + {base}^{exp}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "
        {solution_text} \\ 
        $ {term} + colored({base}^{exp}) = {term} + {power} = {answer} $
    "
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![term, base],
        combinations: t_range.len() * base_range.len(),
    }))
}

/// 13 - 3^2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn sub_power(id: i32, lang: Language) -> Result<Problem> {
    let (base, base_range) = num_gen::integer().range(2, 5).and_random();
    let exp = 2;
    let power = base.pow(exp);
    // Need to make the term larger for a positive answer
    let (term, t_range) = num_gen::integer().range(power + 1, power + 5).and_random();

    let answer = term - power;

    let question = format!("${term} - {base}^{exp}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "
        {solution_text} \\ 
        $ {term} - colored({base}^{exp}) = {term} - {power} = {answer} $
    "
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![term, base],
        combinations: t_range.len() * base_range.len(),
    }))
}

/// 2 * 3^2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn mult_power(id: i32, lang: Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(2, 10).and_random();
    let (base, base_range) = num_gen::integer().range(2, 3).and_random();
    let exp = 2;

    let power = base.pow(exp);
    let answer = factor * power;

    let question = format!("${factor} dot {base}^{exp}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "
        {solution_text} \\ 
        $ {factor} dot colored({base}^{exp}) = {factor} dot {power} = {answer} $
    "
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![factor, base],
        combinations: f_range.len() * base_range.len(),
    }))
}

/// (2 + 3)^2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn par_add_power(id: i32, lang: Language) -> Result<Problem> {
    let (term_1, t_range) = num_gen::integer().range(2, 5).and_random();
    // We need to make sure the sum is at most 10, otherwise the square will get large
    let max_sum = 10;
    let term_2 = num_gen::integer().range(1, max_sum - term_1).random();
    let exp = 2;

    let sum = term_1 + term_2;
    let answer = sum.pow(exp);

    let question = format!("$({term_1} + {term_2})^{exp}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "
        {solution_text} \\ 
        $ colored(({term_1} + {term_2}))^{exp} = {sum}^{exp} = {answer} $
    "
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: term_1,
        combinations: t_range,
    }))
}

/// (12 - 4)^2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn par_sub_power(id: i32, lang: Language) -> Result<Problem> {
    let (term_1, t_range) = num_gen::integer().range(2, 20).and_random();
    let min_diff = 1;
    let max_diff = 10;
    let term_2 = num_gen::integer()
        .range(term_1 - max_diff, term_1 - min_diff)
        .exclude(0)
        .positive();
    let exp = 2;

    let diff = term_1 - term_2;
    let answer = diff.pow(exp);

    let question = format!("$({term_1} - {term_2})^{exp}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "
        {solution_text} \\ 
        $ colored(({term_1} - {term_2}))^{exp} = {diff}^{exp} = {answer} $
    "
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: term_1,
        combinations: t_range,
    }))
}

/// (2 * 3)^2
/// Absolute difficulty: 1
/// Relative difficulty: 4
#[problem]
fn par_mult_power(id: i32, lang: Language) -> Result<Problem> {
    let (factor_1, f_range) = num_gen::integer().range(2, 5).and_random();
    // We need to make sure the product is at most 10, otherwise the square will get large
    let max_product = 10;
    let max_factor = max_product / factor_1.as_i32();
    let factor_2 = num_gen::integer().range(2, max_factor).random();
    let exp = 2;

    let product = factor_1 * factor_2;
    let answer = product.pow(exp);

    let question = format!("$({factor_1} dot {factor_2})^{exp}$");
    let solution_text = get_solution(id, lang)?;
    let solution = format!(
        "
        {solution_text} \\ 
        $ colored(({factor_1} dot {factor_2}))^{exp} = {product}^{exp} = {answer} $
    "
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: factor_1,
        combinations: f_range,
    }))
}

/// 4 * 3^2 + 10
/// Absolute difficulty: 1
/// Relative difficulty: 5
#[problem]
fn mult_power_add(id: i32, _lang: Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(2, 10).and_random();
    let (base, base_range) = num_gen::integer().range(2, 3).and_random();
    let exp = 2;
    let (term, term_range) = num_gen::integer().range(1, 12).and_random();

    let power = base.pow(exp);
    let product = factor * power;
    let answer = product + term;

    let question = format!("${factor} dot {base}^{exp} + {term}$");
    let solution = format!(
        "
        ${factor} dot colored({base}^{exp}) + {term} = colored({factor} dot {power}) + {term} = \\
        = {product} + {term} = {answer}$
"
    );
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![factor, base, term],
        combinations: f_range.len() * base_range.len() * term_range.len(),
    }))
}

/// 4 * 3^2 - 10
/// Absolute difficulty: 1
/// Relative difficulty: 5
#[problem]
fn mult_power_sub(id: i32, _lang: Language) -> Result<Problem> {
    let (factor, f_range) = num_gen::integer().range(2, 10).and_random();
    let (base, base_range) = num_gen::integer().range(2, 3).and_random();
    let exp = 2;

    let power = base.pow(exp);
    let product = factor * power;
    let term = num_gen::integer().range(1, product).random();
    let answer = product - term;

    let question = format!("${factor} dot {base}^{exp} - {term}$");
    let solution = format!(
        "
        ${factor} dot colored({base}^{exp}) - {term} = colored({factor} dot {power}) - {term} = \\
        = {product} - {term} = {answer}$
"
    );
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![factor, base],
        combinations: f_range.len() * base_range.len(),
    }))
}
