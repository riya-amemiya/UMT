//! Tests for the end_of module.

use chrono::{Datelike, TimeZone, Timelike, Utc};
use umt_rust::date::{DateBoundaryUnit, umt_end_of};

#[test]
fn test_sets_milliseconds_to_999_for_second() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 45).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Second);
    assert_eq!(result.nanosecond() / 1_000_000, 999);
    assert_eq!(result.second(), 45);
}

#[test]
fn test_sets_to_59_999_for_minute() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Minute);
    assert_eq!(result.second(), 59);
    assert_eq!(result.nanosecond() / 1_000_000, 999);
}

#[test]
fn test_sets_to_xx_59_59_999_for_hour() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 10, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Hour);
    assert_eq!(result.minute(), 59);
    assert_eq!(result.second(), 59);
    assert_eq!(result.nanosecond() / 1_000_000, 999);
}

#[test]
fn test_sets_to_23_59_59_999_for_day() {
    let input = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Day);
    assert_eq!(result.hour(), 23);
    assert_eq!(result.minute(), 59);
    assert_eq!(result.second(), 59);
    assert_eq!(result.nanosecond() / 1_000_000, 999);
}

#[test]
fn test_returns_saturday_end_for_week() {
    let input = Utc.with_ymd_and_hms(2025, 4, 16, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Week);
    assert_eq!(result.weekday().num_days_from_sunday(), 6);
    assert_eq!(result.hour(), 23);
}

#[test]
fn test_returns_last_day_for_month() {
    let input = Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Month);
    assert_eq!(result.day(), 30);
    assert_eq!(result.month(), 4);
}

#[test]
fn test_returns_feb_28_for_non_leap_february() {
    let input = Utc.with_ymd_and_hms(2025, 2, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Month);
    assert_eq!(result.day(), 28);
}

#[test]
fn test_returns_feb_29_for_leap_february() {
    let input = Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Month);
    assert_eq!(result.day(), 29);
}

#[test]
fn test_returns_last_day_of_q1() {
    let input = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 3);
    assert_eq!(result.day(), 31);
}

#[test]
fn test_returns_last_day_of_q2() {
    let input = Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 6);
    assert_eq!(result.day(), 30);
}

#[test]
fn test_returns_last_day_of_q3() {
    let input = Utc.with_ymd_and_hms(2025, 7, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 9);
    assert_eq!(result.day(), 30);
}

#[test]
fn test_returns_last_day_of_q4() {
    let input = Utc.with_ymd_and_hms(2025, 10, 1, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Quarter);
    assert_eq!(result.month(), 12);
    assert_eq!(result.day(), 31);
}

#[test]
fn test_returns_dec_31_for_year() {
    let input = Utc.with_ymd_and_hms(2025, 6, 15, 0, 0, 0).unwrap();
    let result = umt_end_of(&input, DateBoundaryUnit::Year);
    assert_eq!(result.month(), 12);
    assert_eq!(result.day(), 31);
    assert_eq!(result.hour(), 23);
}
