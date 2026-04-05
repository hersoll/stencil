use anyhow::Result;
use macros::problem;
use math::{Number, Polynomial, Term, num_gen};
use registry::replace_placeholders;
use types::{lang::Language, problems::Problem};
use typst_writer::formatting::{equation_solution, parentheses};

/// Calculate k between (1, 3) and (4, 9) [positive integers]
/// Difficulty: 1
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
        combinations: small_range.len().pow(3),
    })
}

/// Calculate k between (1, -2) and (-5, 3)
/// Difficulty: 2
#[problem]
fn find_k_with_negatives(name: String, lang: &Language) -> Result<Problem> {
    let small_range = num_gen::integer().range(-5, 5).exclude(0);
    let k = small_range.negative(); // Having a negative k ensures we get 
    let x_start = small_range.positive();
    let y_start = small_range.negative();
    // Ensures we get a negative x_end
    let x_step = num_gen::integer().range(-x_start - 5, -x_start).random();
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
        "$ k = (y_2 - y_1)/(x_2 - x_1) =({} - {})/({} - {}) = ({y_step}) / ({x_step}) = {k} $",
        parentheses(y_end),
        parentheses(y_start),
        parentheses(x_end),
        parentheses(x_start),
    );

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![x_start, x_end, y_start, y_end],
        combinations: small_range.len().pow(2),
    })
}

/// Calculate k between (27, 32) and (34, 18)
/// Difficulty: 2
#[problem]
fn find_k_large_integers(name: String, lang: &Language) -> Result<Problem> {
    let k = num_gen::integer().range(-3, 3).random();
    let start_range = num_gen::integer()
        .range(16, 34)
        .exclude_multiple(&[20, 25, 30]);
    let x_start = start_range.random();
    let y_start = start_range.random();
    let x_step = num_gen::integer().range(6, 11).exclude(10).random();
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
        combinations: start_range.len().pow(2),
    })
}

/// Find the equation of the line between (1, 3) and (4, 9) [positive integers]
/// Difficulty: 3
#[problem]
fn find_equation_small_positives(name: String, lang: &Language) -> Result<Problem> {
    let small_range = num_gen::integer().range(1, 5);
    let k = num_gen::integer().range(2, 5).random();
    let m = small_range.random();
    let x_start = small_range.random();
    let y_start = k * x_start + m;
    let x_step = small_range.random();
    let x_end = x_start + x_step;
    let y_end = k * x_end + m;
    let y_step = y_end - y_start;
    let k_term: Term = (k, 'x').into();
    let m_term: Term = m.into();
    let equation: Polynomial = vec![k_term, m_term].into();

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$y = {equation}$");
    let solution =
        format!("$ k = ({y_end} - {y_start})/({x_end} - {x_start}) = {y_step} / {x_step} = {k} $")
            + equation_solution(format!(
                "
                y &= k x + m \\ k = {k} \\
                y &= colored({k})x + m \\ x = {x_start}, y = {y_start} \\
                colored({y_start}) &= {k} dot colored({x_start}) + m \\ \\
                {y_start} &= {mult} + m \\ {m_mult:+}\\
                {m} &= m \\ \\
                    ",
                mult = k * x_start,
                m_mult = -k * x_start
            ))
            .as_str();

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: small_range.len().pow(3),
    })
}

/// Find the equation of the line between (1, -3) and (-4, 9)
/// Difficulty: 4
#[problem]
fn find_equation_with_negatives(name: String, lang: &Language) -> Result<Problem> {
    let k = num_gen::integer()
        .range(-5, 5)
        .exclude_multiple(&[-1, 0, 1])
        .random();
    let small_range = num_gen::integer().range(-5, 5);
    let m = small_range.random();
    let x_start = small_range.positive();
    let y_start = k * x_start + m;
    // Ensures we get a negative x_end
    let x_step = num_gen::integer().range(-x_start - 5, -x_start).random();
    let x_end = x_start + x_step;
    let y_end = k * x_end + m;
    let y_step = y_end - y_start;
    let k_term: Term = (k, 'x').into();
    let m_term: Term = m.into();
    let equation: Polynomial = vec![k_term, m_term].into();

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$y = {equation}$");
    let solution = format!(
        "$ k = ({} - {})/({} - {}) = ({y_step}) / ({x_step}) = {k} $",
        parentheses(y_end),
        parentheses(y_start),
        parentheses(x_end),
        parentheses(x_start),
    ) + equation_solution(format!(
        "
                y &= k x + m \\ k = {k} \\
                y &= colored({k})x + m \\ x = {x_start}, y = {y_start} \\
                colored({y_start}) &= {p_k} dot colored({p_x_start}) + m \\ \\
                {y_start} &= {mult} + m \\ {m_mult:+}\\
                {m} &= m \\ \\
                    ",
        mult = k * x_start,
        m_mult = -k * x_start,
        p_k = parentheses(k),
        p_x_start = parentheses(x_start),
    ))
    .as_str();

    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: small_range.len().pow(3),
    })
}

/// Find the equation of the line between (17, 30) and (29, 54)
/// Difficulty: 4
#[problem]
fn find_equation_large_integers(name: String, lang: &Language) -> Result<Problem> {
    let k = num_gen::integer()
        .range(-3, 3)
        .exclude_multiple(&[-1, 0, 1])
        .random();
    let m = num_gen::integer().range(-10, 10).random();
    let x_start = num_gen::integer().range(11, 29).exclude(20).random();
    let y_start = k * x_start + m;
    let x_step = num_gen::integer().range(6, 11).exclude(10).random();
    let x_end = x_start + x_step;
    let y_end = k * x_end + m;
    let y_step = y_end - y_start;
    let k_term: Term = (k, 'x').into();
    let m_term: Term = m.into();
    let equation: Polynomial = vec![k_term, m_term].into();

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$y = {equation}$");

    let solution = format!(
        "$ k = ({} - {})/({} - {}) = ({y_step}) / ({x_step}) = {k} $",
        parentheses(y_end),
        parentheses(y_start),
        parentheses(x_end),
        parentheses(x_start),
    ) + equation_solution(format!(
        "
                y &= k x + m \\ k = {k} \\
                y &= colored({k})x + m \\ x = {x_start}, y = {y_start} \\
                colored({y_start}) &= {p_k} dot colored({p_x_start}) + m \\ \\
                {y_start} &= {mult} + m \\ {m_mult:+}\\
                {m} &= m \\ \\
                    ",
        mult = k * x_start,
        m_mult = -k * x_start,
        p_k = parentheses(k),
        p_x_start = parentheses(x_start),
    ))
    .as_str();
    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: 30,
    })
}

/// Find the equation of the line between (17, 30) and (29, 30)
/// Difficulty: 4
#[problem]
fn find_equation_k_zero(name: String, lang: &Language) -> Result<Problem> {
    let k = 0;
    let m = num_gen::integer().range(-40, 40).random();
    let x_start = num_gen::integer().range(11, 29).exclude(20).random();
    let y_start = k * x_start + m;
    let x_step = num_gen::integer().range(6, 11).exclude(10).random();
    let x_end = x_start + x_step;
    let y_end = k * x_end + m;
    let y_step = y_end - y_start;
    let k_term: Term = (k, 'x').into();
    let m_term: Term = m.into();
    let equation: Polynomial = vec![k_term, m_term].into();

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$y = {equation}$");

    let solution = format!(
        "$ k = ({} - {})/({} - {}) = ({y_step}) / ({x_step}) = {k} $",
        parentheses(y_end),
        parentheses(y_start),
        parentheses(x_end),
        parentheses(x_start),
    ) + equation_solution(format!(
        "
                y &= k x + m \\ k = {k} \\
                y &= m \\ x = {x_start}, y = {y_start} \\
                colored({y_start}) &= m \\ \\
                {m} &= m \\ \\
                    ",
    ))
    .as_str();
    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: 30,
    })
}

/// Find the equation of the line between (17, 30) and (29, 42)
/// Difficulty: 4
#[problem]
fn find_equation_k_one(name: String, lang: &Language) -> Result<Problem> {
    let k = num_gen::integer().numbers(&[-1, 1]).random();
    let m = num_gen::integer().range(-40, 40).random();
    let x_start = num_gen::integer().range(11, 29).exclude(20).random();
    let y_start = k * x_start + m;
    let x_step = num_gen::integer().range(6, 11).exclude(10).random();
    let x_end = x_start + x_step;
    let y_end = k * x_end + m;
    let y_step = y_end - y_start;
    let k_term: Term = (k, 'x').into();
    let m_term: Term = m.into();
    let equation: Polynomial = vec![k_term.clone(), m_term].into();

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$y = {equation}$");

    let solution = format!(
        "$ k = ({} - {})/({} - {}) = ({y_step}) / ({x_step}) = {k} $",
        parentheses(y_end),
        parentheses(y_start),
        parentheses(x_end),
        parentheses(x_start),
    ) + equation_solution(format!(
        "
                y &= k x + m \\ k = {k} \\
                y &= colored({k_term}) + m \\ x = {x_start}, y = {y_start} \\
                colored({y_start}) &= colored({mult}) + m \\ {m_mult:+}\\
                {m} &= m \\ \\
                    ",
        mult = k * x_start,
        m_mult = -k * x_start,
    ))
    .as_str();
    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k, m],
        combinations: 30,
    })
}

/// Find the equation of the line between (17, 30) and (23, 40)
/// Difficulty: 6
#[problem]
fn find_equation_k_fraction(name: String, lang: &Language) -> Result<Problem> {
    let k: Number = num_gen::fraction()
        .denoms(&[6, 9, 12])
        .max(2)
        .random()
        .into();
    let m: Number = num_gen::fraction()
        .denom(k.denominator().as_i32() / 3) // Ensures integer coordinates
        .min(-3)
        .max(3)
        .random()
        .into();

    // To find the first integer coordinates, we need to do some math.
    // When do we "fill" a sufficient amount of the denominator in k to reach an integer?
    let mut start = m.numerator() * 3; // Since m has a denom that is a third, we start "three steps" up per numerator
    let mut x_start = Number::Integer(0);
    while start % k.denominator() != 0 || k * x_start + m == 0 {
        start += k.numerator();
        x_start = x_start + 1;
    }
    let y_start = (k * x_start + m).simplify();

    let mut x_end = x_start + 1;
    let mut end = start + k.numerator();
    while end % k.denominator() != 0 {
        end += k.numerator();
        x_end = x_end + 1;
    }
    let y_end = (k * x_end + m).simplify();

    let k_term: Term = (k, 'x').into();
    let m_term: Term = m.into();
    let equation: Polynomial = vec![k_term.clone(), m_term].into();

    let problem_data = registry::get_problem_data(&name)?;
    let question_string = problem_data.get_question(lang);
    let question = replace_placeholders(
        question_string,
        &[
            ("p1", format!("$({x_start}, {y_start})$")),
            ("p2", format!("$({x_end}, {y_end})$")),
        ],
    );
    let answer = format!("$y = {equation}$");

    let solution = format!(
        "$ k = ({} - {})/({} - {}) = {k} $",
        parentheses(y_end),
        parentheses(y_start),
        parentheses(x_end),
        parentheses(x_start),
    ) + equation_solution(format!(
        "
                y &= k x + m \\ k = {k} \\
                y &= colored({k_term}) + m \\ x = {x_start}, y = {y_start} \\
                colored({y_start}) &= ({num} dot colored({x_start}))/{denom} + m \\ \\
                {y_start} &= {mult} + m \\ {y_start} = {expanded_y}\\
                colored({expanded_y}) &= {mult} + m \\ {m_mult:+}\\
                m &= {subtracted_fraction} = {m} \\ \\
                    ",
        num = k.numerator(),
        denom = k.denominator(),
        mult = k * x_start,
        m_mult = -k * x_start,
        expanded_y = Number::Fraction(
            (y_start * k.denominator()).as_i32(),
            k.denominator().as_i32()
        ),
        subtracted_fraction = Number::Fraction(
            (y_start * k.denominator() - &(k.numerator() * x_start)).as_i32(),
            k.denominator().as_i32()
        ),
    ))
    .as_str();
    Ok(Problem {
        name,
        question,
        answer,
        solution,
        identifiers: vec![k.numerator().as_i32(), k.denominator().as_i32()],
        combinations: 15,
    })
}
