pub fn to_list_item(s: &String) -> String {
    String::from("+ ") + s
}

pub fn to_heading(heading: &String) -> String {
    String::from("= ") + heading
}

pub fn line_break() -> String {
    String::from("\\")
}

pub fn empty_line() -> String {
    String::from("\n\\\n")
}

pub fn page_break() -> String {
    String::from("\n#pagebreak()\n")
}

pub fn reset_enum() -> String {
    String::from("#item-counter.update(0)\n")
}

/// Formats the solution of an equation, step by step.
///
/// Do not include dollar signs ($) for math formatting. The function makes sure the left column is
/// a block type ($ 2x + 1 $) and the right column is inline ($+2x$).
/// Remember to include empty steps if nothing happens in a step
pub fn equation_solution(equation_string: String) -> String {
    let mut equations: Vec<&str> = Vec::new();
    let mut steps: Vec<&str> = Vec::new();

    for (i, val) in equation_string.split(r#"\"#).enumerate() {
        if i % 2 == 0 {
            equations.push(val);
        } else {
            steps.push(val);
        }
    }

    let combined_equations = equations
        .into_iter()
        .map(|e| format!("$ {} $", e.trim_matches('$').trim()))
        .collect::<Vec<String>>()
        .join(", ");
    let combined_steps = steps
        .into_iter()
        .map(|s| format!("${}$", s.trim_matches('$').trim()))
        .collect::<Vec<String>>()
        .join(", ");

    format!(
        "  #equation-solution(
({combined_equations}),
({combined_steps}),
)"
    )
}
