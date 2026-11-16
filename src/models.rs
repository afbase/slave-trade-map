//! Data model types shared across the application.
//!
//! These structs are used for database query results, component props,
//! and JSON serialization to the JS visualization layer.

use serde::Serialize;
use std::fmt;
use std::str::FromStr;

/// Which dataset(s) to query: Transatlantic, Inter-American, or both combined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DatasetMode {
    Transatlantic,
    InterAmerican,
    Combined,
}

impl DatasetMode {
    pub fn label(&self) -> &'static str {
        match self {
            DatasetMode::Transatlantic => "transatlantic",
            DatasetMode::InterAmerican => "interamerican",
            DatasetMode::Combined => "combined",
        }
    }
}

impl fmt::Display for DatasetMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

impl FromStr for DatasetMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "transatlantic" => Ok(DatasetMode::Transatlantic),
            "interamerican" => Ok(DatasetMode::InterAmerican),
            "combined" => Ok(DatasetMode::Combined),
            _ => Err(()),
        }
    }
}

/// Aggregated origin→destination flow with embarked/disembarked totals.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FlowData {
    pub source_code: i64,
    pub source_name: String,
    pub target_code: i64,
    pub target_name: String,
    pub total_embarked: f64,
    pub total_disembarked: f64,
    pub voyage_count: i64,
    pub first_year: Option<i64>,
    pub last_year: Option<i64>,
    pub dataset: String,
}

/// Summary info for a destination region or port (for map dots and search).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DestinationInfo {
    pub code: i64,
    pub name: String,
    pub total_embarked: f64,
    pub voyage_count: i64,
    pub ta_embarked: f64,
    pub iam_embarked: f64,
}

/// Individual voyage record with multi-stop embarkation/disembarkation detail.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct VoyageRecord {
    pub voyage_id: i64,
    pub ship_name: String,
    pub captain_a: String,
    pub captain_b: String,
    pub captain_c: String,
    pub embark_region_code: i64,
    pub embark_region_name: String,
    pub year_arrived: Option<i64>,
    pub embarked: Option<i64>,
    pub disembarked: Option<i64>,
    pub deaths: Option<i64>,
    // Per-stop embarkation
    pub embark_port1_name: String,
    pub embark_port2_name: String,
    pub embark_port3_name: String,
    pub embarked_port1: Option<i64>,
    pub embarked_port2: Option<i64>,
    pub embarked_port3: Option<i64>,
    // Per-stop disembarkation
    pub disembark_place1_name: String,
    pub disembark_place2_name: String,
    pub disembark_place3_name: String,
    pub disembarked_place1: Option<i64>,
    pub disembarked_place2: Option<i64>,
    pub disembarked_place3: Option<i64>,
    pub dataset: String,
    pub linked_voyage_id: Option<i64>,
}

/// Per-year, per-source breakdown of embarked totals (for timeline chart).
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct YearlySourceFlow {
    pub year: i64,
    pub source_code: i64,
    pub source_name: String,
    pub embarked: f64,
    pub dataset: String,
}

/// An African embarkation place (MJBYPTIMP) with totals for a time range.
/// `is_unspecified` marks the codebook's xx99 "port unspecified" codes.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct OriginInfo {
    pub code: i64,
    pub name: String,
    pub region: i64,
    pub total_embarked: f64,
    pub voyage_count: i64,
    pub is_unspecified: bool,
}
