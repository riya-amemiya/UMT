//! Tests for the is_same module.

use chrono::{DateTime, TimeZone, Utc};
use umt_rust::date::{DateBoundaryUnit, umt_is_same};

fn date(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    milli: i64,
) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .unwrap()
        + chrono::Duration::milliseconds(milli)
}

#[test]
fn test_returns_true_for_identical_timestamps_without_unit() {
    let left = date(2025, 4, 15, 10, 30, 45, 123);
    let right = date(2025, 4, 15, 10, 30, 45, 123);
    assert!(umt_is_same(&left, &right, None));
}

#[test]
fn test_returns_false_for_different_milliseconds_without_unit() {
    let left = date(2025, 4, 15, 10, 30, 45, 123);
    let right = date(2025, 4, 15, 10, 30, 45, 124);
    assert!(!umt_is_same(&left, &right, None));
}

#[test]
fn test_ignores_sub_millisecond_precision_without_unit() {
    let left = date(2025, 4, 15, 10, 30, 45, 123);
    let right = left + chrono::Duration::microseconds(400);
    assert!(umt_is_same(&left, &right, None));
}

#[test]
fn test_returns_true_for_same_second_ignoring_milliseconds() {
    let left = date(2025, 4, 15, 10, 30, 45, 0);
    let right = date(2025, 4, 15, 10, 30, 45, 999);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Second)));
}

#[test]
fn test_returns_false_for_different_seconds() {
    let left = date(2025, 4, 15, 10, 30, 45, 0);
    let right = date(2025, 4, 15, 10, 30, 46, 0);
    assert!(!umt_is_same(&left, &right, Some(DateBoundaryUnit::Second)));
}

#[test]
fn test_returns_true_for_same_minute_ignoring_seconds() {
    let left = date(2025, 4, 15, 10, 30, 0, 0);
    let right = date(2025, 4, 15, 10, 30, 59, 0);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Minute)));
}

#[test]
fn test_returns_true_for_same_hour_ignoring_minutes() {
    let left = date(2025, 4, 15, 10, 0, 0, 0);
    let right = date(2025, 4, 15, 10, 59, 0, 0);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Hour)));
}

#[test]
fn test_returns_true_for_same_day_with_different_times() {
    let left = date(2025, 4, 15, 1, 0, 0, 0);
    let right = date(2025, 4, 15, 23, 59, 0, 0);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Day)));
}

#[test]
fn test_returns_false_for_adjacent_days() {
    let left = date(2025, 4, 15, 0, 0, 0, 0);
    let right = date(2025, 4, 16, 0, 0, 0, 0);
    assert!(!umt_is_same(&left, &right, Some(DateBoundaryUnit::Day)));
}

#[test]
fn test_returns_true_for_same_week_sunday_start() {
    let sunday = date(2025, 4, 13, 0, 0, 0, 0);
    let saturday = date(2025, 4, 19, 0, 0, 0, 0);
    assert!(umt_is_same(
        &sunday,
        &saturday,
        Some(DateBoundaryUnit::Week)
    ));
}

#[test]
fn test_returns_false_across_week_boundary() {
    let saturday = date(2025, 4, 19, 0, 0, 0, 0);
    let sunday = date(2025, 4, 20, 0, 0, 0, 0);
    assert!(!umt_is_same(
        &saturday,
        &sunday,
        Some(DateBoundaryUnit::Week)
    ));
}

#[test]
fn test_returns_true_for_same_month_with_different_days() {
    let left = date(2025, 4, 1, 0, 0, 0, 0);
    let right = date(2025, 4, 30, 0, 0, 0, 0);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Month)));
}

#[test]
fn test_returns_false_for_different_months() {
    let left = date(2025, 4, 15, 0, 0, 0, 0);
    let right = date(2025, 5, 15, 0, 0, 0, 0);
    assert!(!umt_is_same(&left, &right, Some(DateBoundaryUnit::Month)));
}

#[test]
fn test_returns_true_for_same_quarter() {
    let left = date(2025, 4, 1, 0, 0, 0, 0);
    let right = date(2025, 6, 30, 0, 0, 0, 0);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Quarter)));
}

#[test]
fn test_returns_false_for_different_quarters() {
    let left = date(2025, 3, 31, 0, 0, 0, 0);
    let right = date(2025, 4, 1, 0, 0, 0, 0);
    assert!(!umt_is_same(&left, &right, Some(DateBoundaryUnit::Quarter)));
}

#[test]
fn test_returns_true_for_same_year_with_different_months() {
    let left = date(2025, 1, 1, 0, 0, 0, 0);
    let right = date(2025, 12, 31, 0, 0, 0, 0);
    assert!(umt_is_same(&left, &right, Some(DateBoundaryUnit::Year)));
}

#[test]
fn test_returns_false_for_different_years() {
    let left = date(2024, 4, 15, 0, 0, 0, 0);
    let right = date(2025, 4, 15, 0, 0, 0, 0);
    assert!(!umt_is_same(&left, &right, Some(DateBoundaryUnit::Year)));
}
