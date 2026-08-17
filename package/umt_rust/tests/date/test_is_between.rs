//! Tests for the is_between module.

use chrono::{TimeZone, Utc};
use umt_rust::date::{DateBoundaryUnit, DateInclusivity, umt_is_between};

fn dt(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
        .unwrap()
}

#[test]
fn test_returns_true_for_timestamp_strictly_inside_range() {
    let start = dt(2025, 4, 10, 10, 0);
    let mid = dt(2025, 4, 15, 12, 0);
    let end = dt(2025, 4, 20, 18, 0);
    assert!(umt_is_between(
        &mid,
        &start,
        &end,
        None,
        DateInclusivity::Exclusive
    ));
}

#[test]
fn test_returns_false_for_bounds_with_default_exclusivity() {
    let start = dt(2025, 4, 10, 10, 0);
    let end = dt(2025, 4, 20, 18, 0);
    assert!(!umt_is_between(
        &start,
        &start,
        &end,
        None,
        DateInclusivity::Exclusive
    ));
    assert!(!umt_is_between(
        &end,
        &start,
        &end,
        None,
        DateInclusivity::Exclusive
    ));
}

#[test]
fn test_includes_both_bounds_when_inclusive() {
    let start = dt(2025, 4, 10, 10, 0);
    let end = dt(2025, 4, 20, 18, 0);
    assert!(umt_is_between(
        &start,
        &start,
        &end,
        None,
        DateInclusivity::Inclusive
    ));
    assert!(umt_is_between(
        &end,
        &start,
        &end,
        None,
        DateInclusivity::Inclusive
    ));
}

#[test]
fn test_half_open_inclusivity() {
    let start = dt(2025, 4, 10, 10, 0);
    let end = dt(2025, 4, 20, 18, 0);
    assert!(umt_is_between(
        &start,
        &start,
        &end,
        None,
        DateInclusivity::StartInclusive
    ));
    assert!(!umt_is_between(
        &end,
        &start,
        &end,
        None,
        DateInclusivity::StartInclusive
    ));
    assert!(!umt_is_between(
        &start,
        &start,
        &end,
        None,
        DateInclusivity::EndInclusive
    ));
    assert!(umt_is_between(
        &end,
        &start,
        &end,
        None,
        DateInclusivity::EndInclusive
    ));
}

#[test]
fn test_returns_false_when_start_is_after_end() {
    let start = dt(2025, 4, 10, 10, 0);
    let mid = dt(2025, 4, 15, 12, 0);
    let end = dt(2025, 4, 20, 18, 0);
    assert!(!umt_is_between(
        &mid,
        &end,
        &start,
        None,
        DateInclusivity::Exclusive
    ));
}

#[test]
fn test_day_and_week_and_quarter_granularity() {
    assert!(umt_is_between(
        &dt(2025, 4, 15, 23, 59),
        &dt(2025, 4, 15, 0, 0),
        &dt(2025, 4, 15, 1, 0),
        Some(DateBoundaryUnit::Day),
        DateInclusivity::Inclusive
    ));
    assert!(!umt_is_between(
        &dt(2025, 4, 16, 0, 0),
        &dt(2025, 4, 10, 0, 0),
        &dt(2025, 4, 15, 0, 0),
        Some(DateBoundaryUnit::Day),
        DateInclusivity::Inclusive
    ));
    assert!(umt_is_between(
        &dt(2025, 4, 19, 0, 0),
        &dt(2025, 4, 13, 0, 0),
        &dt(2025, 4, 19, 0, 0),
        Some(DateBoundaryUnit::Week),
        DateInclusivity::Inclusive
    ));
    assert!(!umt_is_between(
        &dt(2025, 4, 20, 0, 0),
        &dt(2025, 4, 13, 0, 0),
        &dt(2025, 4, 19, 0, 0),
        Some(DateBoundaryUnit::Week),
        DateInclusivity::Inclusive
    ));
    assert!(umt_is_between(
        &dt(2025, 6, 30, 0, 0),
        &dt(2025, 4, 1, 0, 0),
        &dt(2025, 6, 1, 0, 0),
        Some(DateBoundaryUnit::Quarter),
        DateInclusivity::Inclusive
    ));
    assert!(!umt_is_between(
        &dt(2025, 7, 1, 0, 0),
        &dt(2025, 4, 1, 0, 0),
        &dt(2025, 6, 1, 0, 0),
        Some(DateBoundaryUnit::Quarter),
        DateInclusivity::Inclusive
    ));
}
