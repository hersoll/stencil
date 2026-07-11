//! This module is responsible for reading the logging DBs and calculate different stats from that
//! data.
mod api_counts;
pub mod leaderboards;
mod pdf_counts;
mod problem_set_counts;
pub use api_counts::*;
pub use pdf_counts::*;
pub use problem_set_counts::*;

use serde::Serialize;
use sqlx::postgres::types::PgInterval;

/// Used in certain stat APIs to let the sqlx query be dynamic with regards to time
pub enum AggregationDuration {
    Hours(u16),
    Days(u16),
    AllTime,
}

impl AggregationDuration {
    pub fn to_interval(&self) -> PgInterval {
        match self {
            Self::Hours(h) => PgInterval {
                months: 0,
                days: 0,
                microseconds: (*h as i64) * 60 * 60 * 1_000_000,
            },
            Self::Days(d) => PgInterval {
                months: 0,
                days: *d as i32,
                microseconds: 0,
            },
            Self::AllTime => PgInterval {
                months: 2048,
                days: 0,
                microseconds: 0,
            },
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ReturnData {
    Boolean(bool),
    BigInt(i64),
    Integer(i32),
    SmallInt(i16),
    CharVar(String),
    Null,
}

impl From<String> for ReturnData {
    fn from(value: String) -> Self {
        Self::CharVar(value)
    }
}

impl From<i16> for ReturnData {
    fn from(value: i16) -> Self {
        Self::SmallInt(value)
    }
}

impl From<Option<i16>> for ReturnData {
    fn from(value: Option<i16>) -> Self {
        match value {
            Some(small_int) => Self::SmallInt(small_int),
            None => Self::Null,
        }
    }
}

impl From<i32> for ReturnData {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<Option<i32>> for ReturnData {
    fn from(value: Option<i32>) -> Self {
        match value {
            Some(integer) => Self::Integer(integer),
            None => Self::Null,
        }
    }
}

impl From<i64> for ReturnData {
    fn from(value: i64) -> Self {
        Self::BigInt(value)
    }
}

impl From<bool> for ReturnData {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}

/// Represents the occurences of a certain value in the DB
///
/// Used for queries with `GROUP BY`, where you want counts for multiple variants at the same time, for example how
/// many PDFs are rendered with a title and how many are rendered without one. In that case, the
/// `sqlx` query will return a `Vec<ValueCount>` where the values are of enum `Boolean`.
#[derive(Serialize)]
pub struct ValueCount {
    pub value: ReturnData,
    pub count: i64,
}

/// Used for timeline-style stats
#[derive(Serialize)]
pub struct TimeLineCount {
    pub time: chrono::NaiveDateTime,
    pub count: i64,
}

// # What do we want to do?
//
// ## Averages (with duration):
// - Render time
// - Topics per set
// - Problem count per set
// - Set count per pdf
