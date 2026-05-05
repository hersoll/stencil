use crate::HasReplacements;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use std::{fmt::Display, ops::Deref};

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
            None => Question(String::from("No question in DB!")),
        }
    }
}

impl From<Option<String>> for Answer {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Answer(s),
            None => Answer(String::from("No answer in DB!")),
        }
    }
}

impl From<Option<String>> for Solution {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => Solution(s),
            None => Solution(String::from("No answer in DB!")),
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
