use db::logging::stats::AggregationDuration;
use serde::Deserialize;

pub mod api_counts;
pub mod leaderboards;
pub mod pdf_attributes;
pub mod problem_set_attributes;

/// Used for dynamic router paths, so serde knows that it can deserialize the durations into
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationPath {
    Day,
    Week,
    Month,
    ThreeMonths,
    Year,
    All,
}

impl DurationPath {
    fn as_duration(&self) -> AggregationDuration {
        match self {
            Self::Day => AggregationDuration::Hours(24),
            Self::Week => AggregationDuration::Days(7),
            Self::Month => AggregationDuration::Days(30),
            Self::ThreeMonths => AggregationDuration::Days(90),
            Self::Year => AggregationDuration::Days(365),
            Self::All => AggregationDuration::AllTime,
        }
    }
}
