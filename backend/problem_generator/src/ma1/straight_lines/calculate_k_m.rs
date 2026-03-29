use anyhow::Result;
use macros::problem;
use math::num_gen;
use registry::replace_placeholders;
use types::{lang::Language, problems::Problem};

/// Calculate k between (1, 3) and (4, 9) [positive integers]
/// Difficulty: 2
#[problem]
fn find_k_all_positives(name: String, lang: &Language) -> Result<Problem> {
    let small_range = num_gen::integer().range(1, 5);
    let k = small_range.random();
    let x_start = small_range.random();
    let y_start = small_range.random();
    let x_step = small_range.random();
    let y_step = x_step * k;
    let x_end = x_start + x_step;
    let y_end = y_start + y_step;

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$k = {k}$");
    let solution = format!(
        "$ k = (y_2 - y_1)/(x_2 - x_1) =({y_end} - {y_start})/({x_end} - {x_start}) = {y_step} / {x_step} = {k} $"
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![x_start, x_end, y_start, y_end],
        combinations: small_range.len().pow(4),
    })
}
