use super::{list_item, reformat_newlines};
use crate::typst_file_builder::{DEFAULT_QUESTION_COLUMNS, SetOptions};
use anyhow::Result;
use std::fmt::Write;
use tracing::debug;

/// Formats the sets to columns with equal height
pub fn sets_to_balanced_columns(
    sets: &[Vec<String>],
    group_prefixes: &[Option<String>],
    set_options: &[SetOptions],
    par_spacing: &Option<u8>,
) -> Result<String> {
    let mut out = String::with_capacity(8 * 1024);

    for (i, set) in sets.iter().enumerate() {
        // Write group prefix (if any)
        if let Some(prefix) = group_prefixes.get(i).and_then(|p| p.clone()) {
            writeln!(out, "{prefix}")?;
        }

        writeln!(out, "\n#let problem_set = (")?;
        // Write each list item
        for item in set.iter() {
            writeln!(out, "{}", list_item(item))?;
        }
        writeln!(out, ")")?;

        let spacing_setting = if let Some(spacing) = set_options.get(i).and_then(|o| o.spacing) {
            format!(", custom_spacing: {spacing}mm")
        } else {
            String::new()
        };

        let heading_setting = if let Some(option) = set_options.get(i) {
            if option.heading.is_empty() {
                String::new()
            } else {
                format!(", title: [{}]", reformat_newlines(&option.heading))
            }
        } else {
            String::new()
        };

        // Call the balanced function in Typst
        writeln!(
            out,
            "#context{{balanced({}, problem_set, here().position().y{}{})}}",
            set_options
                .get(i)
                .map(|o| o.question_columns)
                .unwrap_or(DEFAULT_QUESTION_COLUMNS),
            spacing_setting,
            heading_setting
        )?;

        // Paragraph spacing between sets (except after last)
        if i != sets.len().saturating_sub(1) {
            if let Some(spacing) = par_spacing {
                writeln!(out, "#v({}mm)", spacing)?;
            } else {
                writeln!(out, "#v(1.8em)")?;
            }
        }
    }

    debug!(
        "Allocated 8kb for balanced column set. Final length: {}",
        out.len()
    );
    Ok(out)
}

///Writes the set to a flow from one filled column to the next
pub fn sets_to_columns(sets: &[Vec<String>], columns: &u8) -> Result<String> {
    let mut out = String::with_capacity(sets[0].len() * sets.len() * 128);

    writeln!(out, "#columns({columns}, enum(spacing: 2.5em, ")?;
    for set in sets.iter() {
        for entry in set.iter() {
            writeln!(out, "{}", list_item(entry))?;
        }
    }
    writeln!(out, "))")?;
    Ok(out)
}
