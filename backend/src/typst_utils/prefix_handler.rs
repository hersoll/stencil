use crate::{
    Language,
    db::PrefixEntry,
    registry::{PREFIX_DATA, PROBLEM_DATA, RegistryError},
    typst_utils::typst_file_builder::DocumentOptions,
};
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fmt::Write;

type PrefixRegistry = std::sync::RwLockReadGuard<'static, HashMap<i32, PrefixEntry>>;

/// Will either:
///
/// - Create a group prefix for the entire set (if prefixes match)
/// - Group sequential problems as nested lists and give them a group prefix (if specified in DocumentOptions)
/// - Apply prefix text to individual problems
pub fn handle_prefixes(
    question_set: Vec<String>,
    answer_set: Vec<String>,
    group_prefixes: &mut Vec<Option<String>>,
    document_options: &DocumentOptions,
    problem_names: &Vec<String>,
) -> Result<(Vec<String>, Vec<String>)> {
    if problem_names.is_empty() {
        group_prefixes.push(None);
        return Ok((question_set, answer_set));
    }

    let prefix_ids = fetch_prefix_ids(problem_names)?;
    let prefix_reg = get_prefix_registry()?;

    // Case 1: Entire set shares a single prefix.
    //
    // Attach a group prefix, leave the questions and answers untouched
    if let Some(group_prefix) =
        detect_group_prefix(&prefix_ids, &prefix_reg, &document_options.lang)
    {
        group_prefixes.push(Some(group_prefix));
        return Ok((question_set, answer_set));
    }

    // Case 2: No group prefix.
    group_prefixes.push(None);
    match document_options.max_prefix_group {
        // Case 2A: Group similar problems
        //
        // Questions and answers are grouped into nested lists,
        // and therefore will be mutated quite a lot
        Some(max) => apply_grouped_prefixes(
            question_set,
            answer_set,
            &prefix_ids,
            &prefix_reg,
            max,
            &document_options.lang,
        ),
        // Case 2B: Don't group anything
        //
        // Prefixes are applied to the questions that have them.
        // Nothing else is changed
        None => apply_inline_prefixes(
            question_set,
            answer_set,
            &prefix_ids,
            &prefix_reg,
            &document_options.lang,
        ),
    }
}

fn fetch_prefix_ids(problem_names: &[String]) -> Result<Vec<Option<i32>>> {
    let reg = PROBLEM_DATA
        .read()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PROBLEM_DATA".to_string(),
        })?;

    Ok(problem_names
        .iter()
        .map(|id| reg.get(id).and_then(|p| p.prefix_id))
        .collect())
}

fn get_prefix_registry() -> Result<PrefixRegistry> {
    let registry = PREFIX_DATA
        .read()
        .map_err(|_| RegistryError::RegistryMutexIsPoisoned {
            registry: "PREFIX_DATA".to_string(),
        })?;
    Ok(registry)
}

/// Check whether or not every problem shares a prefix, and if so,
/// return the group version of that prefix
fn detect_group_prefix(
    prefix_ids: &[Option<i32>],
    prefix_reg: &HashMap<i32, PrefixEntry>,
    lang: &Language,
) -> Option<String> {
    let first = prefix_ids.first().and_then(|&id| id)?;

    if prefix_ids.iter().all(|&id| id == Some(first)) {
        prefix_reg
            .get(&first)
            .map(|p| format!("{}:", p.get_group_text(lang)))
    } else {
        None
    }
}

/// Takes sequential similar problems, groups them together into
/// a nested list, and gives them a group prefix.
fn apply_grouped_prefixes(
    question_set: Vec<String>,
    answer_set: Vec<String>,
    prefix_ids: &[Option<i32>],
    prefix_reg: &HashMap<i32, PrefixEntry>,
    max_group: u8,
    lang: &Language,
) -> Result<(Vec<String>, Vec<String>)> {
    let groups = group_related_prefixes(prefix_ids, max_group);

    let mut prefixed_questions = Vec::new();
    let mut prefixed_answers = Vec::new();
    let mut idx = 0usize;

    for len in groups {
        if len == 1 {
            push_single_prefixed(
                idx,
                &question_set,
                &answer_set,
                prefix_ids,
                prefix_reg,
                &mut prefixed_questions,
                &mut prefixed_answers,
                lang,
            );
        } else {
            push_grouped_enum(
                idx,
                len as usize,
                &question_set,
                &answer_set,
                prefix_ids,
                prefix_reg,
                &mut prefixed_questions,
                &mut prefixed_answers,
                lang,
            )?;
        }
        idx += len as usize;
    }

    Ok((prefixed_questions, prefixed_answers))
}

/// Group sequential identical prefixes into groups, but cap group length at max_len
/// Example prefix_ids: [Some(1) Some(2) Some(2) Some(2) Some(2) None Some(2) Some(2)]
/// max_len = 4 -> [1 4 1 2] (each value is a group length)
/// max_len = 3 -> [1 3 1 1 2]
fn group_related_prefixes(prefix_ids: &[Option<i32>], max_len: u8) -> Vec<u8> {
    if prefix_ids.len() < 2 {
        return vec![prefix_ids.len() as u8];
    }
    let mut groups = Vec::new();
    let mut current_length = 1u8;
    let mut latest_id = prefix_ids[0];

    for &id in &prefix_ids[1..] {
        if id.is_some() && id == latest_id {
            current_length += 1;
            if current_length > max_len {
                groups.push(max_len);
                current_length = 1;
            }
        } else {
            groups.push(current_length);
            latest_id = id;
            current_length = 1;
        }
    }

    groups.push(current_length);
    groups
}

/// Simply give every problem that wants it a prefix.
///
/// Does not mutate the length of the sets
fn apply_inline_prefixes(
    mut question_set: Vec<String>,
    answer_set: Vec<String>,
    prefix_ids: &[Option<i32>],
    prefix_reg: &HashMap<i32, PrefixEntry>,
    lang: &Language,
) -> Result<(Vec<String>, Vec<String>)> {
    for (i, id) in prefix_ids.iter().enumerate() {
        if let Some(id) = id {
            if let Some(prefix) = prefix_reg.get(id) {
                let text = prefix.get_text(lang);
                question_set[i] = format!("{text} {}", question_set[i]);
            }
        }
    }
    Ok((question_set, answer_set))
}

/// Give a single problem its prefix (if it has one)
/// and appends it to the prefixed list
fn push_single_prefixed(
    idx: usize,
    question_set: &[String],
    answer_set: &[String],
    prefix_ids: &[Option<i32>],
    prefix_reg: &HashMap<i32, PrefixEntry>,
    prefixed_questions: &mut Vec<String>,
    prefixed_answers: &mut Vec<String>,
    lang: &Language,
) {
    let question = match prefix_ids[idx].and_then(|id| prefix_reg.get(&id)) {
        Some(prefix) => {
            let text = prefix.get_text(lang);
            format!("{text} {}", question_set[idx])
        }
        None => question_set[idx].clone(),
    };

    prefixed_questions.push(question);
    prefixed_answers.push(answer_set[idx].clone());
}

/// Give a problem group its prefix, turn it into a nested list
/// and append it to the prefixed list
fn push_grouped_enum(
    idx: usize,
    group_length: usize,
    question_set: &[String],
    answer_set: &[String],
    prefix_ids: &[Option<i32>],
    prefix_reg: &HashMap<i32, PrefixEntry>,
    prefixed_questions: &mut Vec<String>,
    prefixed_answers: &mut Vec<String>,
    lang: &Language,
) -> Result<()> {
    let Some(id) = prefix_ids[idx] else {
        return Ok(());
    };

    let Some(prefix) = prefix_reg.get(&id) else {
        return Err(anyhow!(
            "Got a prefix_id of {id} but it didn't match to a prefix in the PrefixRegistry"
        ));
    };

    let prefix_text = prefix.get_group_text(lang);

    let mut grouped_questions = String::new();
    let mut grouped_answers = String::new();

    // Questions header
    writeln!(
        grouped_questions,
        "{prefix_text}: \n\n#enum(numbering: \"a)\", indent: -0.8em,"
    )?;

    // Answers header
    writeln!(grouped_answers, "#enum(numbering: \"a)\",")?;

    for j in idx..idx + group_length {
        let nested_answer = adjust_nested_answer(&answer_set[j]);
        writeln!(grouped_questions, "[{}],", question_set[j])?;
        writeln!(grouped_answers, "[{}],", nested_answer)?;
    }

    grouped_questions.push(')');
    grouped_answers.push(')');

    prefixed_questions.push(grouped_questions);
    prefixed_answers.push(grouped_answers);

    Ok(())
}

/// To make nested lists have the same width available for solutions (to avoid weird formatting
/// near line breaks) as regular solutions, we need to adjust their insets.
///
/// The rule for this is set in formatting::solution_rules()
/// TODO: The entire nested solution is unbreakable right now.
fn adjust_nested_answer(answer: &String) -> String {
    answer.replace("#solution", "#nested_solution")
}
