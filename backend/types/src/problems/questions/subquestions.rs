use std::fmt::Display;

use crate::problems::Question;

#[derive(Debug, Default)]
pub struct SubQuestions {
    /// Any text or math to be displayed before the subquestions
    pre: Option<String>,

    subquestions: Vec<Question>,

    /// Any text or math to be displayed after the subquestions
    post: Option<String>,
}

impl SubQuestions {
    pub fn pre(&mut self, pre: impl Display) -> &mut Self {
        self.pre = Some(pre.to_string());
        self
    }

    pub fn subquestion(&mut self, question: impl Into<Question>) -> &mut Self {
        self.subquestions.push(question.into());
        self
    }

    pub fn post(&mut self, post: impl Display) -> &mut Self {
        self.post = Some(post.to_string());
        self
    }
}

impl Display for SubQuestions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pre) = &self.pre {
            writeln!(f, "{pre}")?;
        }

        write!(f, "\n{}", subquestions_start())?;
        for question in &self.subquestions {
            write!(f, "{}", subquestion(question))?;
        }
        write!(f, "{}", subquestions_end())?;

        if let Some(post) = &self.post {
            writeln!(f, "{post}")?;
        }
        Ok(())
    }
}

/// Fixed string that can be used whenever a subquestion list needs to be started
pub fn subquestions_start() -> String {
    String::from("#enum(numbering: \"a)\", indent: -0.8em,\n")
}
pub fn subquestion(q: impl Display) -> String {
    format!("[{q}],\n")
}
/// Fixed string that can be used whenever a subquestion list needs to be ended
pub fn subquestions_end() -> String {
    String::from(")\n")
}

impl From<SubQuestions> for Question {
    fn from(questions: SubQuestions) -> Self {
        Question(questions.to_string())
    }
}
