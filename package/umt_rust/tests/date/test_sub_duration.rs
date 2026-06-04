//! Tests for the sub_duration module.

use chrono::{Datelike, TimeZone, Utc};
use umt_rust::date::{DurationUnit, umt_sub_duration};

#[test]
fn test_subtracts_milliseconds() {
    let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 1000, DurationUnit::Ms);
    assert_eq!(result.timestamp_millis(), base.timestamp_millis() - 1000);
}

#[test]
fn test_subtracts_months_clamps_to_last_day() {
    let base = Utc.with_ymd_and_hms(2025, 3, 31, 0, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 1, DurationUnit::Month);
    assert_eq!(result.month(), 2);
    assert_eq!(result.day(), 28);
}

#[test]
fn test_subtracts_years_across_leap_year() {
    let base = Utc.with_ymd_and_hms(2025, 2, 28, 0, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 1, DurationUnit::Y);
    assert_eq!(result.year(), 2024);
    assert_eq!(result.month(), 2);
    assert_eq!(result.day(), 28);
}

#[test]
fn test_subtracts_seconds() {
    let base = Utc.with_ymd_and_hms(2025, 6, 1, 12, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 30, DurationUnit::S);
    assert_eq!(result.timestamp_millis(), base.timestamp_millis() - 30_000);
}

#[test]
fn test_subtracts_minutes() {
    let base = Utc.with_ymd_and_hms(2025, 6, 1, 12, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 5, DurationUnit::M);
    assert_eq!(result.timestamp_millis(), base.timestamp_millis() - 300_000);
}

#[test]
fn test_subtracts_hours() {
    let base = Utc.with_ymd_and_hms(2025, 6, 1, 12, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 1, DurationUnit::H);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() - 3_600_000
    );
}

#[test]
fn test_subtracts_days() {
    let base = Utc.with_ymd_and_hms(2025, 6, 1, 12, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 1, DurationUnit::D);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() - 86_400_000
    );
}

#[test]
fn test_subtracts_weeks() {
    let base = Utc.with_ymd_and_hms(2025, 6, 15, 12, 0, 0).unwrap();
    let result = umt_sub_duration(&base, 1, DurationUnit::W);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() - 7 * 86_400_000
    );
}
