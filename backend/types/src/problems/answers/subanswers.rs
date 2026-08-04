use crate::problems::Answer;
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct SubAnswers {
    subanswers: Vec<Answer>,
    /// Any text or math to be displayed before the subquestions
    pre: Option<String>,
    /// Any text or math to be displayed after the subquestions
    post: Option<String>,
}

impl SubAnswers {
    pub fn pre(&mut self, pre: impl Display) -> &mut Self {
        self.pre = Some(pre.to_string());
        self
    }

    pub fn subanswer(&mut self, answer: impl Into<Answer>) -> &mut Self {
        self.subanswers.push(answer.into());
        self
    }

    pub fn post(&mut self, post: impl Display) -> &mut Self {
        self.post = Some(post.to_string());
        self
    }
}

impl Display for SubAnswers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pre) = &self.pre {
            writeln!(f, "{pre}")?;
        }

        write!(f, "\n{}", subanswers_start())?;
        for answer in &self.subanswers {
            write!(f, "{}", subanswer(answer))?;
        }
        write!(f, "{}", subanswers_end())?;

        if let Some(post) = &self.post {
            writeln!(f, "{post}")?;
        }
        Ok(())
    }
}

pub fn subanswers_start() -> String {
    String::from("#enum(numbering: \"a)\",\n")
}
pub fn subanswer(a: impl Display) -> String {
    let nested = adjust_nested_answer(&a.to_string());
    format!("[{nested}],\n")
}
pub fn subanswers_end() -> String {
    String::from(")\n")
}

/// To make nested lists have the same width available for solutions (to avoid weird formatting
/// near line breaks) as regular solutions, we need to adjust their insets.
///
/// The rule for this is set in formatting::solution_rules()
/// TODO: The entire nested solution is unbreakable right now.
fn adjust_nested_answer(answer: &str) -> String {
    answer.replace("#solution", "#nested_solution")
}

impl From<SubAnswers> for Answer {
    fn from(answers: SubAnswers) -> Self {
        Answer(answers.to_string())
    }
}
