//! Tests for the is_after module.

use chrono::{DateTime, TimeZone, Utc};
use umt_rust::date::{DateBoundaryUnit, umt_is_after};

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
fn test_returns_true_when_later_without_unit() {
    assert!(umt_is_after(
        &date(2025, 4, 15, 10, 30, 45, 124),
        &date(2025, 4, 15, 10, 30, 45, 123),
        None
    ));
}

#[test]
fn test_returns_false_for_identical_timestamps_without_unit() {
    let value = date(2025, 4, 15, 10, 30, 45, 123);
    assert!(!umt_is_after(&value, &value, None));
}

#[test]
fn test_returns_false_when_earlier_without_unit() {
    assert!(!umt_is_after(
        &date(2025, 4, 15, 0, 0, 0, 0),
        &date(2025, 4, 16, 0, 0, 0, 0),
        None
    ));
}

#[test]
fn test_returns_false_within_same_day() {
    assert!(!umt_is_after(
        &date(2025, 4, 15, 23, 59, 0, 0),
        &date(2025, 4, 15, 1, 0, 0, 0),
        Some(DateBoundaryUnit::Day)
    ));
}

#[test]
fn test_returns_true_for_next_day() {
    assert!(umt_is_after(
        &date(2025, 4, 16, 0, 0, 0, 0),
        &date(2025, 4, 15, 23, 59, 0, 0),
        Some(DateBoundaryUnit::Day)
    ));
}

#[test]
fn test_returns_false_within_same_sunday_start_week() {
    assert!(!umt_is_after(
        &date(2025, 4, 19, 0, 0, 0, 0),
        &date(2025, 4, 13, 0, 0, 0, 0),
        Some(DateBoundaryUnit::Week)
    ));
}

#[test]
fn test_returns_true_across_week_boundary() {
    assert!(umt_is_after(
        &date(2025, 4, 20, 0, 0, 0, 0),
        &date(2025, 4, 19, 0, 0, 0, 0),
        Some(DateBoundaryUnit::Week)
    ));
}

#[test]
fn test_compares_month_quarter_and_year() {
    assert!(umt_is_after(
        &date(2025, 5, 1, 0, 0, 0, 0),
        &date(2025, 4, 30, 0, 0, 0, 0),
        Some(DateBoundaryUnit::Month)
    ));
    assert!(!umt_is_after(
        &date(2025, 6, 30, 0, 0, 0, 0),
        &date(2025, 4, 1, 0, 0, 0, 0),
        Some(DateBoundaryUnit::Quarter)
    ));
    assert!(umt_is_after(
        &date(2026, 1, 1, 0, 0, 0, 0),
        &date(2025, 12, 31, 0, 0, 0, 0),
        Some(DateBoundaryUnit::Year)
    ));
}

#[test]
fn test_compares_second_minute_and_hour() {
    assert!(!umt_is_after(
        &date(2025, 4, 15, 10, 30, 45, 999),
        &date(2025, 4, 15, 10, 30, 45, 0),
        Some(DateBoundaryUnit::Second)
    ));
    assert!(umt_is_after(
        &date(2025, 4, 15, 10, 31, 0, 0),
        &date(2025, 4, 15, 10, 30, 59, 0),
        Some(DateBoundaryUnit::Minute)
    ));
    assert!(!umt_is_after(
        &date(2025, 4, 15, 10, 59, 0, 0),
        &date(2025, 4, 15, 10, 0, 0, 0),
        Some(DateBoundaryUnit::Hour)
    ));
}
