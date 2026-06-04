//! Tests for the is_same_day module.

use chrono::{TimeZone, Utc};
use umt_rust::date::umt_is_same_day;

#[test]
fn test_returns_true_for_same_date_different_times() {
    let a = Utc.with_ymd_and_hms(2025, 4, 15, 1, 0, 0).unwrap();
    let b = Utc.with_ymd_and_hms(2025, 4, 15, 23, 59, 0).unwrap();
    assert!(umt_is_same_day(&a, &b));
}

#[test]
fn test_returns_false_for_adjacent_days() {
    let a = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let b = Utc.with_ymd_and_hms(2025, 4, 16, 0, 0, 0).unwrap();
    assert!(!umt_is_same_day(&a, &b));
}

#[test]
fn test_returns_false_for_different_month() {
    let a = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let b = Utc.with_ymd_and_hms(2025, 5, 15, 0, 0, 0).unwrap();
    assert!(!umt_is_same_day(&a, &b));
}

#[test]
fn test_returns_false_for_different_year() {
    let a = Utc.with_ymd_and_hms(2024, 4, 15, 0, 0, 0).unwrap();
    let b = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    assert!(!umt_is_same_day(&a, &b));
}
