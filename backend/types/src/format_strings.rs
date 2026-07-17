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

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum SplitFormatted<T: HasReplacements> {
    Single(T),
    Multiple { index: usize, splits: Vec<T> },
}

/// Newtype that simply contains a [`String`].
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct Question(String);

/// Newtype that simply contains a [`String`].
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct Answer(String);

/// Newtype that simply contains a [`String`].
#[derive(Debug, Clone, Deserialize, Serialize, Type)]
#[sqlx(transparent)]
pub struct Solution(String);

impl Deref for Question {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Answer {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for Solution {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl HasReplacements for Question {
    fn get_str(&self) -> &String {
        &self.0
    }
}
impl HasReplacements for Answer {
    fn get_str(&self) -> &String {
        &self.0
    }
}
impl HasReplacements for Solution {
    fn get_str(&self) -> &String {
        &self.0
    }
}

impl From<Option<String>> for Question {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Question(s),
            None => Question(String::new()),
        }
    }
}

impl From<Option<String>> for Answer {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Answer(s),
            None => Answer(String::new()),
        }
    }
}

impl From<Option<String>> for Solution {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Solution(s),
            None => Solution(String::new()),
        }
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
