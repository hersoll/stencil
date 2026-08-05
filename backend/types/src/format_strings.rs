use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use std::{fmt::Display, ops::Deref};

/// Trait for structs with a [`String`] that contains `{placeholders}`
///
/// Questions, Answers and Solutions might have template strings in them which will be replaced at
/// runtime. This trait enables you to retrieve the strings and replace the templating.
pub trait HasReplacements {
    /// Accesses the templated [`String`] used for replacement.
    fn get_str(&self) -> &String;

    /// Takes a template string containing `{key}` and replaces that key with `value`.
    ///
    /// Used in problems with dynamic text questions, for example:
    /// `"Use the function {f} to solve..."`
    fn replace_one(&self, key: &str, value: impl Display) -> String {
        let placeholder = format!("{{{}}}", key);
        self.get_str().replace(&placeholder, &value.to_string())
    }

    /// Takes a template string containing `{key}` patterns and replaces those keys with values.
    ///
    /// Used in problems with dynamic text questions, for example:
    /// `"Use the function {f} to solve..."`
    fn replace_multiple(&self, key_value_pairs: &[(&str, impl Display)]) -> String {
        let mut result = self.get_str().to_owned();
        for (key, value) in key_value_pairs {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }
        result
    }
}

pub trait HasSubdivisions: HasReplacements {
    const SUB_MARKER: &str = r"\sub";
    const POST_MARKER: &str = r"\post";
    fn to_subdivisions(&self) -> Subdivisions {
        let mut pre: Option<Subdivision> = None;
        let mut post: Option<Subdivision> = None;
        // Extract post, leave the rest
        let subs_with_pre =
            if let Some((before_post, post_part)) = self.get_str().split_once(Self::POST_MARKER) {
                post = Some(post_part.into());
                before_post
            } else {
                self.get_str()
            };
        // Extract pre, leave the subs
        //
        // NOTE: if the string starts with SUB_MARKER, the pre will become Some("") and consume the
        // first marker. This is expected, to make the split() in the return actually work
        // (otherwise the first subdivision becomes "")
        let subs = if let Some((pre_part, subs_part)) = subs_with_pre.split_once(Self::SUB_MARKER) {
            pre = Some(pre_part.into());
            subs_part
        } else {
            subs_with_pre
        };

        Subdivisions {
            latest_index: 0,
            pre,
            post,
            subdivisions: subs
                .split(Self::SUB_MARKER)
                .map(Subdivision::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Subdivision(String);

pub struct Subdivisions {
    pre: Option<Subdivision>,
    subdivisions: Vec<Subdivision>,
    post: Option<Subdivision>,
    latest_index: usize,
}

impl Subdivisions {
    const NOT_FOUND: &str = "NOT FOUND";
    pub fn pre(&self) -> Subdivision {
        self.pre
            .clone()
            .unwrap_or(Subdivision::from(Self::NOT_FOUND))
    }

    pub fn sub(&mut self) -> Subdivision {
        let sub = self
            .subdivisions
            .get(self.latest_index)
            .unwrap_or(&Subdivision::from(Self::NOT_FOUND))
            .clone();
        self.latest_index += 1;
        sub
    }

    pub fn post(&self) -> Subdivision {
        self.post
            .clone()
            .unwrap_or(Subdivision::from(Self::NOT_FOUND))
    }
}

/// Newtype that simply contains a [`String`].
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct QuestionString(String);

/// Newtype that simply contains a [`String`].
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct AnswerString(String);

/// Newtype that simply contains a [`String`].
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct SolutionString(String);

impl Deref for QuestionString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for AnswerString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SolutionString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HasReplacements for QuestionString {
    fn get_str(&self) -> &String {
        &self.0
    }
}
impl HasReplacements for AnswerString {
    fn get_str(&self) -> &String {
        &self.0
    }
}
impl HasReplacements for SolutionString {
    fn get_str(&self) -> &String {
        &self.0
    }
}

impl HasSubdivisions for QuestionString {}
impl HasSubdivisions for AnswerString {}
impl HasSubdivisions for SolutionString {}

impl HasReplacements for Subdivision {
    fn get_str(&self) -> &String {
        &self.0
    }
}

impl From<&str> for Subdivision {
    fn from(s: &str) -> Self {
        Subdivision(s.to_string())
    }
}

impl From<String> for QuestionString {
    fn from(value: String) -> Self {
        QuestionString(value)
    }
}
impl From<String> for AnswerString {
    fn from(value: String) -> Self {
        AnswerString(value)
    }
}
impl From<String> for SolutionString {
    fn from(value: String) -> Self {
        SolutionString(value)
    }
}

impl From<Option<String>> for QuestionString {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => QuestionString(s),
            None => QuestionString(String::new()),
        }
    }
}

impl From<Option<String>> for AnswerString {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => AnswerString(s),
            None => AnswerString(String::new()),
        }
    }
}

impl From<Option<String>> for SolutionString {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => SolutionString(s),
            None => SolutionString(String::new()),
        }
    }
}

impl Display for QuestionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for AnswerString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for SolutionString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Subdivision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
