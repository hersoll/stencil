//! This module is responsible for reading the logging DBs and calculate different stats from that
//! data.
mod api_counts;
pub use api_counts::*;

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

/// Used for timeline-style stats
#[derive(Serialize)]
pub struct HourlyCount {
    pub hour: chrono::NaiveDateTime,
    pub count: i64,
}

/// Used for timeline-style stats
#[derive(Serialize)]
pub struct DailyCount {
    pub day: chrono::NaiveDateTime,
    pub count: i64,
}

/// Used for timeline-style stats
#[derive(Serialize)]
pub struct WeeklyCount {
    pub week_start: chrono::NaiveDateTime,
    pub count: i64,
}

// # What do we want to do?
//
// - For EVERY column in stats_pdf and stats_sets, show a pie chart (with time selection)
// - (Rename DB tables to logs_*)
// - Pie chart over topics per set, set count per pdf
//
// ## Leaderboards:
// - Most used chapters
// - Most used topics
// - Most excluded problems
// - Best and worst render times
// - Busiest days
//
// ## Averages (with timeline):
// - Render time
// - Topics per set
// - Problem count per set
// - Set count per pdf
