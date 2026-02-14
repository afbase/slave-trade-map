//! Utility functions for search and parsing.

use crate::models::DatasetMode;

/// Fuzzy-match `query` against `target`, returning a score if all query chars appear in order.
///
/// - Exact substring match: base 100 (+ 50 if prefix match, - target length penalty)
/// - Subsequence match: +3 per consecutive char, +1 per non-consecutive
/// - Returns `None` if query is empty or not a subsequence of target
pub fn fuzzy_score(query: &str, target: &str) -> Option<i32> {
    let q = query.to_lowercase();
    let t = target.to_lowercase();
    if q.is_empty() {
        return None;
    }
    if t.contains(&q) {
        let mut score = 100;
        if t.starts_with(&q) {
            score += 50;
        }
        score -= t.len() as i32;
        return Some(score);
    }
    let mut qi = q.chars().peekable();
    let mut score = 0i32;
    let mut prev_matched = false;
    for tc in t.chars() {
        if let Some(&qc) = qi.peek() {
            if tc == qc {
                qi.next();
                score += if prev_matched { 3 } else { 1 };
                prev_matched = true;
            } else {
                prev_matched = false;
            }
        }
    }
    if qi.peek().is_some() {
        None
    } else {
        Some(score)
    }
}

/// Parse a dataset mode string ("transatlantic", "interamerican", "combined") into `DatasetMode`.
pub fn dataset_mode_from_str(s: &str) -> DatasetMode {
    match s {
        "interamerican" => DatasetMode::InterAmerican,
        "combined" => DatasetMode::Combined,
        _ => DatasetMode::Transatlantic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuzzy_empty_query_returns_none() {
        assert_eq!(fuzzy_score("", "Jamaica"), None);
    }

    #[test]
    fn fuzzy_exact_match_high_score() {
        let score = fuzzy_score("Jamaica", "Jamaica").unwrap();
        assert!(score > 100, "exact match should score > 100, got {}", score);
    }

    #[test]
    fn fuzzy_prefix_match_beats_substring() {
        let prefix = fuzzy_score("Jam", "Jamaica").unwrap();
        let substr = fuzzy_score("aic", "Jamaica").unwrap();
        assert!(prefix > substr, "prefix {} should beat substring {}", prefix, substr);
    }

    #[test]
    fn fuzzy_no_match_returns_none() {
        assert_eq!(fuzzy_score("xyz", "Jamaica"), None);
    }

    #[test]
    fn fuzzy_subsequence_match() {
        assert!(fuzzy_score("jmc", "Jamaica").is_some());
    }

    #[test]
    fn fuzzy_case_insensitive() {
        let upper = fuzzy_score("JAMAICA", "jamaica");
        let lower = fuzzy_score("jamaica", "JAMAICA");
        assert_eq!(upper, lower);
    }

    #[test]
    fn dataset_mode_parsing() {
        assert_eq!(dataset_mode_from_str("transatlantic"), DatasetMode::Transatlantic);
        assert_eq!(dataset_mode_from_str("interamerican"), DatasetMode::InterAmerican);
        assert_eq!(dataset_mode_from_str("combined"), DatasetMode::Combined);
        assert_eq!(dataset_mode_from_str("unknown"), DatasetMode::Transatlantic);
    }
}
