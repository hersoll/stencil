use super::{list_item, reformat_newlines};
use crate::typst_file_builder::{AnswerSet, QuestionSet};
use anyhow::Result;
use std::fmt::Write;
use types::pdf::{QuestionSetFormattingOptions, SanitizedTypstString};

/// Formats the sets to columns with equal height
pub fn questions_to_balanced_columns(
    sets: &[QuestionSet],
    group_prefixes: &[Option<String>],
    formatting_options: &[QuestionSetFormattingOptions],
    par_spacing: &Option<u8>,
) -> Result<String> {
    let mut out = String::with_capacity(8 * 1024);
    let default_options = QuestionSetFormattingOptions::default();

    for (i, set) in sets.iter().enumerate() {
        let set_option = formatting_options.get(i).unwrap_or(&default_options);

        writeln!(out, "\n#let problem_set = (")?;
        // Write each list item
        for item in set.questions.iter() {
            writeln!(out, "{}", list_item(item))?;
        }
        writeln!(out, ")")?;

        let spacing_setting =
            if let Some(spacing) = formatting_options.get(i).and_then(|o| o.spacing) {
                format!(", custom_spacing: {spacing}mm")
            } else {
                String::new()
            };

        // Custom headings always overwrite prefixes.
        if let Some(SanitizedTypstString(custom_heading)) = &set_option.heading {
            writeln!(out, "{}", reformat_newlines(custom_heading))?;
        } else if let Some(group_prefix) = group_prefixes.get(i).unwrap_or(&None).as_ref() {
            writeln!(out, "{group_prefix}")?;
        }

        // Call the balanced function in Typst
        writeln!(
            out,
            "#context{{balanced({}, problem_set, here().position().y{})}}",
            set_option.question_columns, spacing_setting
        )?;

        // Add pagebreak, or paragraph spacing between sets (except after last)
        let final_set_index = sets.len().saturating_sub(1);
        if i == final_set_index { // Don't add anything after the final set
        } else if set_option.pagebreak_after {
            writeln!(out, "{}", super::page_break())?;
        } else {
            if let Some(spacing) = par_spacing {
                writeln!(out, "#v({}mm)", spacing)?;
            } else {
                writeln!(out, "#v(1.8em)")?;
            }
        }
    }

    Ok(out)
}

///Writes the set to a flow from one filled column to the next
pub fn answers_to_columns(sets: &[AnswerSet], columns: &u8) -> Result<String> {
    let mut out = String::with_capacity(sets[0].answers.len() * sets.len() * 128);

    writeln!(out, "#columns({columns}, enum(spacing: 2.5em, ")?;
    for set in sets.iter() {
        for entry in set.answers.iter() {
            writeln!(out, "{}", list_item(entry))?;
        }
    }
    writeln!(out, "))")?;
    Ok(out)
}
