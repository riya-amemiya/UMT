//! Tests for the is_weekend module.

use chrono::{TimeZone, Utc};
use umt_rust::date::umt_is_weekend;

#[test]
fn test_returns_true_on_saturday() {
    let input = Utc.with_ymd_and_hms(2025, 4, 19, 0, 0, 0).unwrap();
    assert!(umt_is_weekend(&input));
}

#[test]
fn test_returns_true_on_sunday() {
    let input = Utc.with_ymd_and_hms(2025, 4, 20, 0, 0, 0).unwrap();
    assert!(umt_is_weekend(&input));
}

#[test]
fn test_returns_false_on_monday() {
    let input = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
    assert!(!umt_is_weekend(&input));
}

#[test]
fn test_returns_false_on_friday() {
    let input = Utc.with_ymd_and_hms(2025, 4, 18, 0, 0, 0).unwrap();
    assert!(!umt_is_weekend(&input));
}
