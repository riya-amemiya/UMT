//! Tests for the diff module.

use chrono::{TimeZone, Utc};
use umt_rust::date::{DurationUnit, umt_diff};

#[test]
fn test_computes_day_difference() {
    let left = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::D), 364);
}

#[test]
fn test_computes_hour_difference() {
    let left = Utc.with_ymd_and_hms(2025, 1, 1, 5, 0, 0).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::H), 5);
}

#[test]
fn test_computes_minute_difference() {
    let left = Utc.with_ymd_and_hms(2025, 1, 1, 0, 30, 0).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::M), 30);
}

#[test]
fn test_computes_second_difference() {
    let left = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 30).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::S), 30);
}

#[test]
fn test_computes_millisecond_difference() {
    let left = Utc
        .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
        .unwrap()
        .with_timezone(&Utc)
        + chrono::Duration::milliseconds(500);
    let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::Ms), 500);
}

#[test]
fn test_computes_week_difference() {
    let left = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::W), 2);
}

#[test]
fn test_computes_positive_month_difference_adjusting_for_incomplete_month() {
    let right = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
    let left1 = Utc.with_ymd_and_hms(2025, 3, 14, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left1, &right, DurationUnit::Month), 1);
    let left2 = Utc.with_ymd_and_hms(2025, 3, 16, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left2, &right, DurationUnit::Month), 2);
}

#[test]
fn test_computes_negative_month_difference_adjusting_for_incomplete_month() {
    let left = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
    let right1 = Utc.with_ymd_and_hms(2025, 3, 14, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right1, DurationUnit::Month), -1);
    let right2 = Utc.with_ymd_and_hms(2025, 3, 16, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right2, DurationUnit::Month), -2);
}

#[test]
fn test_respects_time_of_day_for_month_boundary_positive() {
    let left = Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 1, 15, 11, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::Month), 0);
}

#[test]
fn test_respects_time_of_day_for_month_boundary_negative() {
    let left = Utc.with_ymd_and_hms(2025, 1, 15, 11, 0, 0).unwrap();
    let right = Utc.with_ymd_and_hms(2025, 2, 15, 10, 0, 0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::Month), 0);
}

#[test]
fn test_computes_year_difference() {
    let a = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
    let b = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&a, &b, DurationUnit::Y), 1);
    assert_eq!(umt_diff(&b, &a, DurationUnit::Y), -1);
}

#[test]
fn test_returns_zero_for_identical_dates() {
    let date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_diff(&date, &date, DurationUnit::D), 0);
    assert_eq!(umt_diff(&date, &date, DurationUnit::Month), 0);
    assert_eq!(umt_diff(&date, &date, DurationUnit::Y), 0);
}

#[test]
fn test_truncates_fractional_fixed_unit_differences_toward_zero() {
    let left = Utc.timestamp_millis_opt(1500).unwrap();
    let right = Utc.timestamp_millis_opt(0).unwrap();
    assert_eq!(umt_diff(&left, &right, DurationUnit::S), 1);
    assert_eq!(umt_diff(&right, &left, DurationUnit::S), -1);
}
