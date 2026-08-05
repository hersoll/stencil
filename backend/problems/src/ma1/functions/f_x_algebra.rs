use anyhow::Result;
use macros::problem;
use math::{
    Evaluable, Number, Term,
    functions::Function,
    num_gen::{self, NumberGenerator},
    symbols::{self, X},
};
use types::{
    format_strings::{HasReplacements, HasSubdivisions},
    lang::Language,
    problems::{Answer, Problem, ProblemParameters, Question, Solution},
};
use typst_writer::formatting::equation_solution;

// In this module, problems in the form of f(3) is known as "calculating y"
// and problems like f(x) = 3 are known as "calculating x"

/// y = 3x + 1, x = 3
/// Absolute difficulty: 1
/// Relative difficulty: 1
#[problem]
fn without_notation_y(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(2, 10).and_random();
    let (m, m_range) = num_gen::integer().range(-10, 10).exclude(0).and_random();
    let x_value = num_gen::integer().range(1, 5).random();
    let y_value = k * x_value + m;

    let mut expression = Function::linear(k, m).without_function_notation();
    let question = registry::get_question(id, lang)?.replace_multiple(&[
        ("expression", expression.to_string()),
        ("x", x_value.to_string()),
    ]);

    expression = expression.aligned();
    let mut solution = Solution::with_steps();
    solution.line(&expression).step(format!("x = {x_value}"));
    let replacement = [(symbols::X, &x_value)];
    solution
        .line(expression.print_replacements(&replacement))
        .line(expression.print_evaluation_by_parts(&replacement))
        .aligned("y", y_value);

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$y = {}$", y_value),
        solution: solution.to_string(),
        identifiers: vec![k, m],
        combinations: k_range.len() * m_range.len(),
    }))
}

/// y = 2x + 2, y = 2
/// Absolute difficulty: 2
/// Relative difficulty: 2
#[problem]
fn without_notation_x(id: i32, lang: Language) -> Result<Problem> {
    let (coefficient, coefficient_range) = num_gen::integer().range(2, 10).and_random();
    let (constant, constant_range) = num_gen::integer().range(-10, 10).exclude(0).and_random();
    let answer = num_gen::integer().range(1, 5).random();
    let y = coefficient * answer + constant;

    let expression = format!("y = {}x {:+}", coefficient, constant);
    let question = registry::get_question(id, lang)?
        .replace_multiple(&[("expression", expression), ("y", y.to_string())]);

    let solution = equation_solution(&format!(
        "y &= {coefficient}x {constant:+} \\ y={y} \\
        {y} &= {coefficient}x {constant:+} \\ {sub_constant} \\
        {lhs} &= {coefficient}x \\ div {coefficient} \\
        {answer} &= x \\ ",
        sub_constant = typst_writer::formatting::subtract_number(constant),
        lhs = answer * coefficient
    ));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x = {answer}$"),
        solution,
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    }))
}

/// f(3), no negatives
/// Absolute difficulty: 4
/// Relative difficulty: 3
#[problem]
fn find_y_no_negatives(id: i32, lang: Language) -> Result<Problem> {
    let (coefficient, coefficient_range) = num_gen::integer().range(2, 10).and_random();
    let x = num_gen::integer().range(1, 5).random();
    let (constant, constant_range) = num_gen::integer()
        .range((-x * coefficient).max(Number::Integer(-10)), 10)
        .exclude(0)
        .and_random();
    let y = coefficient * x + constant;

    let expression = format!("f(x) = {}x {:+}", coefficient, constant);
    let question = registry::get_question(id, lang)?
        .replace_multiple(&[("expression", expression), ("x", x.to_string())]);

    let solution = format!(
        "f(x) &= {coefficient}x {constant:+} \\x={x} \\
       f(colored({x})) &= {coefficient} dot colored({x}) {constant:+} \\ \\
       f({x}) &= {prod} {constant:+} \\ \\
       f({x}) &= {y} \\",
        prod = x * coefficient
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$f({x}) = {y}$"),
        solution: equation_solution(&solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    }))
}

/// Find x where f(x) = 2
/// Absolute difficulty: 4
/// Relative difficulty: 4
#[problem]
fn find_x_where_f_x(id: i32, lang: Language) -> Result<Problem> {
    let (coefficient, coefficient_range) = num_gen::integer().range(2, 10).and_random();
    let x = num_gen::integer().range(1, 5).random();
    let (constant, constant_range) = num_gen::integer()
        .range((-x * coefficient).max(Number::Integer(-10)), 10)
        .exclude(0)
        .and_random();
    let y = coefficient * x + constant;

    let expression = format!("f(x) = {}x {:+}", coefficient, constant);
    let question = registry::get_question(id, lang)?
        .replace_multiple(&[("expression", expression), ("y", y.to_string())]);

    let solution = format!(
        "f(x) &= {coefficient}x {constant:+} \\f(x)={y} \\
       colored({y}) &= {coefficient}x {constant:+} \\ {sub_con}\\
              {y_c} &= {coefficient}x \\ {div_coef}\\
       {x} &= x \\",
        sub_con = typst_writer::formatting::subtract_number(constant),
        div_coef = typst_writer::formatting::divide_number(coefficient),
        y_c = y - constant
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("$x = {x}$"),
        solution: equation_solution(&solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    }))
}

/// Solve the equation f(x) = 4
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn equation_f_x_equals(id: i32, lang: Language) -> Result<Problem> {
    let (coefficient, coefficient_range) = num_gen::integer()
        .range(-10, 10)
        .exclude_multiple(&[-1, 0, 1])
        .and_random();
    let x = num_gen::integer().range(-7, 7).random();
    let (constant, constant_range) = num_gen::integer().range(-10, 10).exclude(0).and_random();
    let y = coefficient * x + constant;
    let f_name = symbols::get_function_name()?;
    let var = symbols::get_variable()?;

    let expression = format!("{f_name}({var}) = {coefficient}{var} {constant:+}");
    let question = registry::get_question(id, lang)?.replace_multiple(&[
        ("expression", expression),
        ("y", y.to_string()),
        ("var", var.to_string()),
        ("f", f_name.to_string()),
    ]);

    let solution = format!(
        "{f_name}({var}) &= {coefficient}{var} {constant:+} \\{f_name}({var})={y} \\
       colored({y}) &= {coefficient}{var} {constant:+} \\ {sub_con}\\
              {y_c} &= {coefficient}{var} \\ {div_coef}\\
       {x} &= {var} \\",
        sub_con = typst_writer::formatting::subtract_number(constant),
        div_coef = typst_writer::formatting::divide_number(coefficient),
        y_c = y - constant
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${var} = {x}$"),
        solution: equation_solution(&solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    }))
}

/// f(-3)
/// Absolute difficulty: 4
/// Relative difficulty: 5
#[problem]
fn find_y(id: i32, lang: Language) -> Result<Problem> {
    let (coefficient, coefficient_range) = num_gen::integer()
        .range(-10, 10)
        .exclude_multiple(&[-1, 0, 1])
        .and_random();
    let x = num_gen::integer().range(-7, 7).random();
    let (constant, constant_range) = num_gen::integer().range(-10, 10).exclude(0).and_random();
    let y = coefficient * x + constant;
    let f_name = symbols::get_function_name()?;
    let var = symbols::get_variable()?;

    let expression = format!("{f_name}({var}) = {coefficient}{var} {constant:+}");
    let question = registry::get_question(id, lang)?.replace_multiple(&[
        ("expression", expression),
        ("x", x.to_string()),
        ("var", var.to_string()),
        ("f", f_name.to_string()),
    ]);

    let solution = format!(
        "{f_name}({var}) &= {coefficient}{var} {constant:+} \\{var}={x} \\
           {f_name}(colored({x})) &= {par_coef} dot colored({par_x}) {constant:+} \\ \\
           {f_name}({x}) &= {prod} {constant:+} \\ \\
           {f_name}({x}) &= {y} \\",
        prod = x * coefficient,
        par_coef = typst_writer::formatting::parentheses(&coefficient),
        par_x = typst_writer::formatting::parentheses(&x),
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer: format!("${f_name}({x}) = {y}$"),
        solution: equation_solution(&solution),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    }))
}

/// f(x) = 2x - 1
/// a) f(3)     b) f(x) = 4
/// Absolute difficulty: 4
/// Relative difficulty: 6
#[problem]
fn combination_x_y(id: i32, lang: Language) -> Result<Problem> {
    let (coefficient, coefficient_range) = num_gen::integer()
        .range(-10, 10)
        .exclude_multiple(&[-1, 0, 1])
        .and_random();
    let x_range = num_gen::integer().range(-7, 7);
    let x_1 = x_range.random();
    let x_2 = x_range.random();
    let (constant, constant_range) = num_gen::integer().range(-10, 10).exclude(0).and_random();
    let y_1 = coefficient * x_1 + constant;
    let y_2 = coefficient * x_2 + constant;
    let f_name = symbols::get_function_name()?;
    let var = symbols::get_variable()?;

    let f_x = format!("{f_name}({var})");
    let f_x_1 = format!("{f_name}({x_1})");

    let expression = format!("${f_x} = {coefficient}{var} {constant:+}$");
    let mut question_strs = registry::get_question(id, lang)?.to_subdivisions();
    let mut question = Question::subquestions();
    question
        .pre(question_strs.pre().replace_one("f", expression))
        .subquestion(question_strs.sub().replace_one("eq", format!("${f_x_1}$")))
        .subquestion(
            question_strs
                .sub()
                .replace_one("eq", format!("${f_x} = {y_2}$")),
        );

    let mut answer = Answer::subanswers();
    answer
        .subanswer(format!("${f_x_1} = {y_1}$"))
        .subanswer(format!("${var} = {x_2}$"));

    let first_solution = equation_solution(&format!(
        "{f_name}({var}) &= {coefficient}{var} {constant:+} \\{var}={x_1} \\
           {f_name}(colored({x_1})) &= {par_coef} dot colored({par_x}) {constant:+} \\ \\
           {f_x_1} &= {prod} {constant:+} \\ \\
           {f_x_1} &= {y_1} \\",
        prod = x_1 * coefficient,
        par_coef = typst_writer::formatting::parentheses(&coefficient),
        par_x = typst_writer::formatting::parentheses(&x_1),
    ));

    let second_solution = equation_solution(&format!(
        "{f_x} &= {coefficient}{var} {constant:+} \\{f_x}={y_2} \\
       colored({y_2}) &= {coefficient}{var} {constant:+} \\ {sub_con}\\
              {y_c} &= {coefficient}{var} \\ {div_coef}\\
       {x_2} &= {var} \\",
        sub_con = typst_writer::formatting::subtract_number(constant),
        div_coef = typst_writer::formatting::divide_number(coefficient),
        y_c = y_2 - constant
    ));

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution: format!("{first_solution} \\ {second_solution}"),
        identifiers: vec![coefficient, constant],
        combinations: coefficient_range.len() * constant_range.len(),
    }))
}

/// f(x) = 2x + 4. Bestäm f(a+1)
/// Absolute difficulty: 7
/// Relative difficulty: 7
#[problem]
fn insert_algebra_positive(id: i32, lang: Language) -> Result<Problem> {
    let (function_coefficient, function_coefficient_range) =
        num_gen::integer().range(2, 6).and_random();
    let (algebra_coefficient, algebra_coefficient_range) =
        num_gen::integer().range(1, 6).and_random();
    let function_constant = num_gen::integer().range(1, 8).random();
    let algebra_constant = num_gen::integer().range(1, 8).random();
    let f_name = symbols::get_function_name()?;
    let algebra_symbol = symbols::get_unknown_with_exclusions(["x", "y"])?;

    let function_term1: Term = (function_coefficient, X).into();
    let function_term2: Term = function_constant.into();
    let function_expression = function_term1.and(&function_term2);
    let algebra_term1: Term = (algebra_coefficient, algebra_symbol).into();
    let algebra_term2: Term = algebra_constant.into();
    let algebra_expression = algebra_term1.and(&algebra_term2);

    let function_string = format!("{f_name}({X}) = {function_expression}");
    let algebra_string = format!("{f_name}({algebra_expression})");
    let question = registry::get_question(id, lang)?
        .replace_multiple(&[("function", function_string), ("algebra", algebra_string)]);
    let answer =
        function_coefficient * algebra_expression.clone() + Term::from_num(function_constant);

    let solution = format!(
        "${f_name}({X}) &= {function_expression} \\
        {f_name}(colored({algebra_expression})) &= {function_coefficient}(colored({algebra_expression})) {function_constant:+} \\
        {f_name}({algebra_expression}) &= {mult_algebra} {function_constant:+} \\
        {f_name}({algebra_expression}) &= {answer}$",
        mult_algebra = function_coefficient * algebra_expression.clone(),
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![function_coefficient, algebra_coefficient],
        combinations: function_coefficient_range.len() * algebra_coefficient_range.len(),
    }))
}

/// f(x) = 4 - 2x. Bestäm f(2a-1)
/// Absolute difficulty: 7
/// Relative difficulty: 8
#[problem]
fn insert_algebra_negative(id: i32, lang: Language) -> Result<Problem> {
    let (function_coefficient, function_coefficient_range) =
        num_gen::integer().range(-6, -2).and_random();
    let (algebra_coefficient, algebra_coefficient_range) =
        num_gen::integer().range(1, 6).and_random();
    let function_constant = num_gen::integer().range(1, 8).random();
    let algebra_constant = num_gen::integer().range(-8, 8).exclude(0).random();
    let f_name = symbols::get_function_name()?;
    let algebra_symbol = symbols::get_unknown_with_exclusions(["x", "y"])?;

    let function_term1: Term = (function_coefficient, X).into();
    let function_term2: Term = function_constant.into();
    let function_expression = function_term1.and(&function_term2).simplify();
    let algebra_term1: Term = (algebra_coefficient, algebra_symbol).into();
    let algebra_term2: Term = algebra_constant.into();
    let algebra_expression = algebra_term1.and(&algebra_term2);

    let function_string = format!("{f_name}({X}) = {function_expression}");
    let algebra_string = format!("{f_name}({algebra_expression})");
    let question = registry::get_question(id, lang)?
        .replace_multiple(&[("function", function_string), ("algebra", algebra_string)]);
    let answer = (function_coefficient * algebra_expression.clone()
        + Term::from_num(function_constant))
    .simplify();

    let solution = format!(
        "${f_name}({X}) &= {function_expression} \\
        {f_name}(colored({algebra_expression})) &= {function_constant} {function_coefficient:+}(colored({algebra_expression}))\\
        {f_name}({algebra_expression}) &= {function_constant}{mult_algebra:+} \\
        {f_name}({algebra_expression}) &= {answer}$",
        mult_algebra = function_coefficient * algebra_expression.clone(),
    );

    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![function_coefficient, algebra_coefficient],
        combinations: function_coefficient_range.len() * algebra_coefficient_range.len(),
    }))
}

// f(x) = 3x - 2. Find the value of f(f(4))
/// Absolute difficulty: 8
/// Relative difficulty: 9
#[problem]
fn insert_number_twice(id: i32, lang: Language) -> Result<Problem> {
    let (k, k_range) = num_gen::integer().range(2, 5).and_random();
    let (m, m_range) = num_gen::integer().range(-6, 6).exclude(0).and_random();
    let (val, val_range) = num_gen::integer().range(-2, 2).and_random();
    let function_symbol = symbols::get_function_name()?;
    let variable = symbols::get_variable()?;
    let function = Function::linear(k, m)
        .with_variable(variable)
        .with_name(function_symbol)
        .with_function_notation();
    let first_replacement = [(variable, &val)];
    let first_step = function.evaluate(&first_replacement);
    let second_replacement = [(variable, &first_step)];
    let second_step = function.evaluate(&second_replacement);

    let problem_data = registry::get_problem_data(id)?;
    let question = problem_data.get_question(lang).replace_multiple(&[
        ("function", function.to_string()),
        (
            "evaluation",
            format!("{function_symbol}({function_symbol}({val}))"),
        ),
    ]);
    let answer = format!("${function_symbol}({function_symbol}({val})) = {second_step}$");
    let solution_hint = problem_data.get_solution(lang);
    let solution = format!(
        "{solution_hint} \\ \\ ${first_evaluation} = {first_step} \\
        {function_symbol}({function_symbol}({val})) = {second_evaluation} = {second_step}$",
        first_evaluation = function.print_replacements(&first_replacement),
        second_evaluation = function.print_replacements(&second_replacement)
    );
    Ok(Problem::from(ProblemParameters {
        id,
        question,
        answer,
        solution,
        identifiers: vec![k, m, val],
        combinations: k_range.len() * m_range.len() * val_range.len(),
    }))
}
