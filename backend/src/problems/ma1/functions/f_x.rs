use std::collections::HashMap;

use crate::{
    IntRange, Polynomial, Problem, Result, Term,
    problems::symbols,
    replace_placeholders,
    typst_formatting::{self, equation_solution},
};
use macros::problem;

// In this module, problems in the form of f(3) is known as "calculating y"
// and problems like f(x) = 3 are known as "calculating x"

/// y = 3x + 1, x = 3
/// Difficulty: 0
#[problem]
fn without_notation_y(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let y = coefficient * x + constant;

    let expression = format!("y = {}x {:+}", coefficient, constant);
    let map = HashMap::from([("expression", expression), ("x", x.to_string())]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);

    let solution = format!(
        "y &= {coefficient}x {constant:+} \\x={x} \\
       y &= {coefficient} dot colored({x}) {constant:+} \\ \\
       y &= {prod} {constant:+} \\ \\
       y &= {y} \\",
        prod = x * coefficient
    );

    let problem = Problem {
        id,
        question,
        answer: format!("$y = {}$", y),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// y = 2x + 2, y = 2
/// Difficulty: 1
#[problem]
fn without_notation_x(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let answer = IntRange::without_zero(1, 5)?.random();
    let y = coefficient * answer + constant;

    let expression = format!("y = {}x {:+}", coefficient, constant);
    let map = HashMap::from([("expression", expression), ("y", y.to_string())]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);

    let solution = equation_solution(format!(
        "y &= {coefficient}x {constant:+} \\ y={y} \\
        {y} &= {coefficient}x {constant:+} \\ {sub_constant} \\
        {lhs} &= {coefficient}x \\ div {coefficient} \\
        {answer} &= x \\ ",
        sub_constant = typst_formatting::subtract_number(constant),
        lhs = answer * coefficient
    ));

    let problem = Problem {
        id,
        question,
        answer: format!("$x = {}$", answer),
        solution,
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// f(3), no negatives
/// Difficulty: 2
#[problem]
fn find_y_no_negatives(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let (constant, constant_range) =
        IntRange::without_zero((-x * coefficient).max(-10), 10)?.and_random();
    let y = coefficient * x + constant;

    let expression = format!("f(x) = {}x {:+}", coefficient, constant);
    let map = HashMap::from([("expression", expression), ("x", x.to_string())]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);

    let solution = format!(
        "f(x) &= {coefficient}x {constant:+} \\x={x} \\
       f(colored({x})) &= {coefficient} dot colored({x}) {constant:+} \\ \\
       f({x}) &= {prod} {constant:+} \\ \\
       f({x}) &= {y} \\",
        prod = x * coefficient
    );

    let problem = Problem {
        id,
        question,
        answer: format!("$f({x}) = {y}$"),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// Find x where f(x) = 2
/// Difficulty: 3
#[problem]
fn find_x_where_f_x(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_zero(2, 10)?.and_random();
    let x = IntRange::without_zero(1, 5)?.random();
    let (constant, constant_range) =
        IntRange::without_zero((-x * coefficient).max(-10), 10)?.and_random();
    let y = coefficient * x + constant;

    let expression = format!("f(x) = {}x {:+}", coefficient, constant);
    let map = HashMap::from([("expression", expression), ("y", y.to_string())]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);

    let solution = format!(
        "f(x) &= {coefficient}x {constant:+} \\f(x)={y} \\
       colored({y}) &= {coefficient}x {constant:+} \\ {sub_con}\\
              {y_c} &= {coefficient}x \\ {div_coef}\\
       {x} &= x \\",
        sub_con = typst_formatting::subtract_number(constant),
        div_coef = typst_formatting::divide_number(coefficient),
        y_c = y - constant
    );

    let problem = Problem {
        id,
        question,
        answer: format!("$x = {x}$"),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// Solve the equation f(x) = 4
/// Difficulty: 3
#[problem]
fn equation_f_x_equals(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_ones_and_zero(-10, 10)?.and_random();
    let x = IntRange::with_zero(-7, 7)?.random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let y = coefficient * x + constant;
    let f_name = symbols::get_function_name()?;
    let var = symbols::get_variable()?;

    let expression = format!("{f_name}({var}) = {coefficient}{var} {constant:+}");
    let map = HashMap::from([
        ("expression", expression),
        ("y", y.to_string()),
        ("var", var.to_string()),
        ("f", f_name.to_string()),
    ]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);

    let solution = format!(
        "{f_name}({var}) &= {coefficient}{var} {constant:+} \\{f_name}({var})={y} \\
       colored({y}) &= {coefficient}{var} {constant:+} \\ {sub_con}\\
              {y_c} &= {coefficient}{var} \\ {div_coef}\\
       {x} &= {var} \\",
        sub_con = typst_formatting::subtract_number(constant),
        div_coef = typst_formatting::divide_number(coefficient),
        y_c = y - constant
    );

    let problem = Problem {
        id,
        question,
        answer: format!("${var} = {x}$"),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// f(-3)
/// Difficulty: 3
#[problem]
fn find_y(id: String, lang: &str) -> Result<Problem> {
    let (coefficient, coefficient_range) = IntRange::without_ones_and_zero(-10, 10)?.and_random();
    let x = IntRange::with_zero(-7, 7)?.random();
    let (constant, constant_range) = IntRange::without_zero(-10, 10)?.and_random();
    let y = coefficient * x + constant;
    let f_name = symbols::get_function_name()?;
    let var = symbols::get_variable()?;

    let expression = format!("{f_name}({var}) = {coefficient}{var} {constant:+}");
    let map = HashMap::from([
        ("expression", expression),
        ("x", x.to_string()),
        ("var", var.to_string()),
        ("f", f_name.to_string()),
    ]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);

    let solution = format!(
        "{f_name}({var}) &= {coefficient}{var} {constant:+} \\{var}={x} \\
           {f_name}(colored({x})) &= {par_coef} dot colored({par_x}) {constant:+} \\ \\
           {f_name}({x}) &= {prod} {constant:+} \\ \\
           {f_name}({x}) &= {y} \\",
        prod = x * coefficient,
        par_coef = typst_formatting::parentheses(coefficient),
        par_x = typst_formatting::parentheses(x),
    );

    let problem = Problem {
        id,
        question,
        answer: format!("${f_name}({x}) = {y}$"),
        solution: equation_solution(solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    };
    Ok(problem)
}

/// f(x) = 2x + 4. Bestäm f(a+1)
/// Difficulty: 6
#[problem]
fn insert_algebra_positive(id: String, lang: &str) -> Result<Problem> {
    let (function_coefficient, function_coefficient_range) =
        IntRange::without_ones_and_zero(2, 6)?.and_random();
    let (algebra_coefficient, algebra_coefficient_range) =
        IntRange::without_zero(1, 6)?.and_random();
    let function_constant = IntRange::without_zero(1, 8)?.random();
    let algebra_constant = IntRange::without_zero(1, 8)?.random();
    let f_name = symbols::get_function_name()?;
    let var = 'x';
    let algebra_symbol = symbols::get_unknown_with_exclusions(['x', 'y'])?;

    let function_term1: Term = (function_coefficient, var).into();
    let function_term2: Term = function_constant.into();
    let function_expression: Polynomial = vec![function_term1, function_term2].into();
    let algebra_term1: Term = (algebra_coefficient, algebra_symbol).into();
    let algebra_term2: Term = algebra_constant.into();
    let algebra_expression: Polynomial = vec![algebra_term1, algebra_term2].into();

    let function_string = format!("{f_name}({var}) = {function_expression}");
    let algebra_string = format!("{f_name}({algebra_expression})");
    let map = HashMap::from([("function", function_string), ("algebra", algebra_string)]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);
    let answer = function_coefficient * algebra_expression.clone() + function_constant;

    let solution = format!(
        "$ {f_name}({var}) &= {function_expression} \\
        {f_name}(colored({algebra_expression})) &= {function_coefficient}(colored({algebra_expression})) {function_constant:+} \\
        {f_name}({algebra_expression}) &= {mult_algebra} {function_constant:+} \\
        {f_name}({algebra_expression}) &= {answer} $",
        mult_algebra = function_coefficient * algebra_expression.clone(),
    );

    let problem = Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![function_coefficient, algebra_coefficient],
        combinations: function_coefficient_range.len() * algebra_coefficient_range.len(),
    };
    Ok(problem)
}

/// f(x) = 4 - 2x. Bestäm f(2a-1)
/// Difficulty: 7
#[problem]
fn insert_algebra_negative(id: String, lang: &str) -> Result<Problem> {
    let (function_coefficient, function_coefficient_range) =
        IntRange::without_ones_and_zero(-6, -2)?.and_random();
    let (algebra_coefficient, algebra_coefficient_range) =
        IntRange::without_zero(1, 6)?.and_random();
    let function_constant = IntRange::without_zero(1, 8)?.random();
    let algebra_constant = IntRange::without_zero(-8, 8)?.random();
    let f_name = symbols::get_function_name()?;
    let var = 'x';
    let algebra_symbol = symbols::get_unknown_with_exclusions(['x', 'y'])?;

    let function_term1: Term = (function_coefficient, var).into();
    let function_term2: Term = function_constant.into();
    let mut function_expression: Polynomial = vec![function_term1, function_term2].into();
    function_expression = function_expression.simplify();
    let algebra_term1: Term = (algebra_coefficient, algebra_symbol).into();
    let algebra_term2: Term = algebra_constant.into();
    let algebra_expression: Polynomial = vec![algebra_term1, algebra_term2].into();

    let function_string = format!("{f_name}({var}) = {function_expression}");
    let algebra_string = format!("{f_name}({algebra_expression})");
    let map = HashMap::from([("function", function_string), ("algebra", algebra_string)]);
    let strings = crate::get_parsed_problem(&id, lang)?;
    let question = replace_placeholders(&strings.question, &map);
    let answer = (function_coefficient * algebra_expression.clone() + function_constant).simplify();

    let solution = format!(
        "$ {f_name}({var}) &= {function_expression} \\
        {f_name}(colored({algebra_expression})) &= {function_constant} {function_coefficient:+}(colored({algebra_expression}))\\
        {f_name}({algebra_expression}) &= {function_constant}{mult_algebra:+} \\
        {f_name}({algebra_expression}) &= {answer} $",
        mult_algebra = function_coefficient * algebra_expression.clone(),
    );

    let problem = Problem {
        id,
        question,
        answer: format!("${answer}$"),
        solution,
        identifiers: vec![function_coefficient, algebra_coefficient],
        combinations: function_coefficient_range.len() * algebra_coefficient_range.len(),
    };
    Ok(problem)
}

// f(x) = 3x - 2. Bestäm f(f(4))
// #[problem]
// fn insert_number_twice(id: String, _lang: &str) -> Result<Problem> {
//     let (coef, coef_range) = IntRange::without_ones_and_zero(2, 5)?.and_random();
//     let (constant, const_range) = IntRange::without_zero(-6, 6)?.and_random();
//     let (val, val_range) = IntRange::without_zero(-8, 8)?.and_random();
//     let function: Expression = vec![
//         (coef, 'x').into(),
//         constant.into()
//     ].into();
//     let first_step = function.evaluate(&vec![('x', val)]);
//
// }
