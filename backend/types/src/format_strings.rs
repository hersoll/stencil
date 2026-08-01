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
    /// Takes a template string containing `{key}` patterns and replaces those keys with values.
    ///
    /// Used in problems with dynamic text questions, for example:
    /// `"Use the function {f} to solve..."`
    fn replace_placeholders(&self, key_value_pairs: &[(&str, impl Display)]) -> String {
        let mut result = self.get_str().to_owned();
        for (key, value) in key_value_pairs {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value.to_string());
        }
        result
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
