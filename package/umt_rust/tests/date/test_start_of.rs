//! Tests for the start_of module.

use chrono::{Datelike, TimeZone, Timelike, Utc};
use umt_rust::date::{DateBoundaryUnit, umt_start_of};

#[test]
fn test_zeros_milliseconds_for_second() {
    let input = Utc
        .with_ymd_and_hms(2025, 4, 15, 10, 30, 45)
        .unwrap()
        .with_nanosecond(123_000_000)
        .unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Second);
    assert_eq!(result.nanosecond() / 1_000_000, 0);
    assert_eq!(result.second(), 45);
}

#[test]
fn test_zeros_seconds_for_minute() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 45).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Minute);
    assert_eq!(result.second(), 0);
    assert_eq!(result.nanosecond() / 1_000_000, 0);
}

#[test]
fn test_zeros_minutes_for_hour() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 45).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Hour);
    assert_eq!(result.minute(), 0);
    assert_eq!(result.second(), 0);
}

#[test]
fn test_zeros_hours_for_day() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Day);
    assert_eq!(result.hour(), 0);
    assert_eq!(result.minute(), 0);
    assert_eq!(result.second(), 0);
}

#[test]
fn test_returns_sunday_for_week() {
    let input = Utc.with_ymd_and_hms(2025, 4, 16, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Week);
    assert_eq!(result.weekday().num_days_from_sunday(), 0);
    assert_eq!(result.hour(), 0);
}

#[test]
fn test_returns_first_day_for_month() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Month);
    assert_eq!(result.day(), 1);
    assert_eq!(result.month(), 4);
    assert_eq!(result.hour(), 0);
}

#[test]
fn test_returns_first_day_of_quarter_for_january() {
    let input = Utc.with_ymd_and_hms(2025, 1, 31, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 1);
    assert_eq!(result.day(), 1);
}

#[test]
fn test_returns_first_day_of_quarter_for_may() {
    let input = Utc.with_ymd_and_hms(2025, 5, 31, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 4);
    assert_eq!(result.day(), 1);
}

#[test]
fn test_returns_first_day_of_quarter_for_august() {
    let input = Utc.with_ymd_and_hms(2025, 8, 31, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 7);
    assert_eq!(result.day(), 1);
}

#[test]
fn test_returns_first_day_of_quarter_for_november() {
    let input = Utc.with_ymd_and_hms(2025, 11, 30, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 10);
    assert_eq!(result.day(), 1);
}

#[test]
fn test_returns_jan_1_for_year() {
    let input = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
    let result = umt_start_of(&input, DateBoundaryUnit::Year);
    assert_eq!(result.month(), 1);
    assert_eq!(result.day(), 1);
    assert_eq!(result.hour(), 0);
}
