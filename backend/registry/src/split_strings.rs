use anyhow::{Result, anyhow, ensure};
use db::{ProblemEntry, ProblemTexts, ProblemTranslations};

/// Some questions, answers and solutions have several text variations. These are stored within the
/// same row (as one long string) in the DB, formatted like:
///
///         "prefix [s1 || s2 || s3] affix [s4 || s5 || s6] suffix".
///
/// This enum is used to store all of these variations in the HashMap with separate strings and keep
/// track of which string was last used, to ensure a nice distribution across variants. In the above
/// example there are just three variants, since s1 and s4 are linked, and so on.
///
/// NOTE: Currently the program shuffles the problems during ordering, which means that keeping track of the index like this is unneccesary.
/// Also note that the current implementation (without the shuffle) means that if two people access the same problem ID at the
/// same time, they might increment the index inbetween each other and the result might not look
/// cyclical.
#[derive(Debug, Clone)]
pub enum SplitProblemEntry {
    Single(ProblemEntry),
    Multiple {
        entry: ProblemEntry,
        latest_index: usize,
        split_texts: Vec<ProblemTranslations>,
    },
}

impl TryFrom<ProblemEntry> for SplitProblemEntry {
    type Error = anyhow::Error;
    fn try_from(value: ProblemEntry) -> Result<Self> {
        // Variant count will keep track of how many variants there are, and will make sure the split_string
        // function errors if the count differs between strings.
        let mut variant_count = 0;
        let sv_questions = split_string(&value.translations.sv.question, &mut variant_count)?;
        let sv_answers = split_string(&value.translations.sv.answer, &mut variant_count)?;
        let sv_solutions = split_string(&value.translations.sv.solution, &mut variant_count)?;
        let en_questions = split_string(&value.translations.en.question, &mut variant_count)?;
        let en_answers = split_string(&value.translations.en.answer, &mut variant_count)?;
        let en_solutions = split_string(&value.translations.en.solution, &mut variant_count)?;

        if variant_count == 0 {
            return Ok(SplitProblemEntry::Single(value));
        }

        // Picks variant `i` from a split field, falling back to the original
        // text if that particular field wasn't split into variants.
        let pick = |splits: &[String], original: &String, i: usize| -> String {
            splits.get(i).cloned().unwrap_or_else(|| original.clone())
        };

        let split_texts = (0..variant_count)
            .map(|i| ProblemTranslations {
                sv: ProblemTexts {
                    question: pick(&sv_questions, &value.translations.sv.question, i).into(),
                    answer: pick(&sv_answers, &value.translations.sv.answer, i).into(),
                    solution: pick(&sv_solutions, &value.translations.sv.solution, i).into(),
                },
                en: ProblemTexts {
                    question: pick(&en_questions, &value.translations.en.question, i).into(),
                    answer: pick(&en_answers, &value.translations.en.answer, i).into(),
                    solution: pick(&en_solutions, &value.translations.en.solution, i).into(),
                },
            })
            .collect();

        Ok(SplitProblemEntry::Multiple {
            entry: value,
            latest_index: 0,
            split_texts,
        })
    }
}

// fn is_split_string(s: &str) -> bool {
//     if let Some(first_bracker) = s.find("[")
//         && let Some(second_bracket) = s.find("]")
//         && first_bracker < second_bracket
//     {
//         true
//     } else {
//         false
//     }
// }

/// Split the string on its [s1 || s2] parts.
///
/// variant_count specifies how many variants there MUST be, if the argument != 0
fn split_string(s: &str, variant_count: &mut usize) -> Result<Vec<String>> {
    let mut index_to_slice_at = 0;
    let mut affixes: Vec<&str> = Vec::new();
    let mut split_parts: Vec<Vec<&str>> = Vec::new();
    while let Some(first_bracket) = s[index_to_slice_at..].find("[")
        && let Some(second_bracket) = s[index_to_slice_at..].find("]")
        && first_bracket < second_bracket
    {
        // The indices first_bracket and second_bracket are relative to the sliced string.
        // We can't look at s inside this loop since the indices will be out of sync
        let slice = &s[index_to_slice_at..];
        let variants: Vec<&str> = slice[(first_bracket + 1)..second_bracket]
            .split(" || ")
            .collect();
        if variants.len() == 1 {
            return Err(anyhow!(
                "No ' || ' (with spaces) inside brackets in split: {s}"
            ));
        }
        split_parts.push(variants);
        // Everything before the [] is pushed to the affixes
        affixes.push(&slice[..first_bracket]);

        // Find more brackets after the point we ended on
        index_to_slice_at += second_bracket + 1;
    }
    if index_to_slice_at < s.len() {
        affixes.push(&s[index_to_slice_at..]);
    }

    // Every split MUST have the same number of variants
    ensure!(
        split_parts
            .iter()
            .all(|parts| parts.len() == split_parts.first().unwrap().iter().len()),
        "Encountered a split string where splits had different number of variants: {s}, first: {:#?}",
        split_parts.first()
    );

    let current_variant_count = split_parts
        .first()
        .unwrap_or(&Vec::<&str>::new())
        .iter()
        .len();

    // Zeroes are okay, otherwise the current variant count MUST match variant_count
    ensure!(
        *variant_count == 0
            || current_variant_count == 0
            || current_variant_count == *variant_count,
        "The split {s} has a different amount of splits than an earlier string."
    );

    let mut out = Vec::new();
    for variant in 0..current_variant_count {
        let mut variant_vec = Vec::new();
        // Do once for each [] block (i).
        // `variant` is the index of the splits:
        // [0 || 1 || 2]
        for (i, split) in split_parts.iter().enumerate() {
            variant_vec.push(affixes[i]);
            variant_vec.push(split[variant]);
        }
        variant_vec.push(affixes.last().unwrap());

        out.push(variant_vec.join(""));
    }

    if current_variant_count != 0 {
        *variant_count = current_variant_count;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_when_no_split() -> Result<()> {
        let input = "This string has no split!";
        assert_eq!(split_string(input, &mut 0)?, Vec::<String>::new());
        Ok(())
    }

    #[test]
    fn splits_with_one_correct_split_into_two() -> Result<()> {
        let input = "I want to [split || share] this.";
        let expected = vec![
            String::from("I want to split this."),
            String::from("I want to share this."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn splits_with_one_correct_split_into_three() -> Result<()> {
        let input = "I want to [split || share || keep] this.";
        let expected = vec![
            String::from("I want to split this."),
            String::from("I want to share this."),
            String::from("I want to keep this."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn splits_with_two_correct_splits() -> Result<()> {
        let input = "The [dog || cat] [barks || meows].";
        let expected = vec![
            String::from("The dog barks."),
            String::from("The cat meows."),
        ];
        assert_eq!(split_string(input, &mut 0)?, expected);
        Ok(())
    }

    #[test]
    fn errors_on_incorrect_split() {
        let input = "This string has no || in its [brackets]!";
        assert!(split_string(input, &mut 0).is_err());
    }
}
