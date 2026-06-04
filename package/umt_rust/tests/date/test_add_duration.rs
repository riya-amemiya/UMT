//! Tests for the add_duration module.

use chrono::{DateTime, Datelike, TimeZone, Utc};
use umt_rust::date::{DurationUnit, umt_add_duration};

fn base_utc() -> DateTime<Utc> {
    Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap()
}

#[test]
fn test_adds_milliseconds() {
    let base = base_utc();
    let result = umt_add_duration(&base, 500, DurationUnit::Ms);
    assert_eq!(result.timestamp_millis(), base.timestamp_millis() + 500);
}

#[test]
fn test_adds_seconds() {
    let base = base_utc();
    let result = umt_add_duration(&base, 30, DurationUnit::S);
    assert_eq!(result.timestamp_millis(), base.timestamp_millis() + 30_000);
}

#[test]
fn test_adds_minutes() {
    let base = base_utc();
    let result = umt_add_duration(&base, 5, DurationUnit::M);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() + 5 * 60_000
    );
}

#[test]
fn test_adds_hours() {
    let base = base_utc();
    let result = umt_add_duration(&base, 2, DurationUnit::H);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() + 2 * 3_600_000
    );
}

#[test]
fn test_adds_days() {
    let base = base_utc();
    let result = umt_add_duration(&base, 7, DurationUnit::D);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() + 7 * 86_400_000
    );
}

#[test]
fn test_adds_weeks() {
    let base = base_utc();
    let result = umt_add_duration(&base, 2, DurationUnit::W);
    assert_eq!(
        result.timestamp_millis(),
        base.timestamp_millis() + 14 * 86_400_000
    );
}

#[test]
fn test_adds_months_clamps_to_last_day() {
    let base = Utc.with_ymd_and_hms(2025, 1, 31, 0, 0, 0).unwrap();
    let result = umt_add_duration(&base, 1, DurationUnit::Month);
    assert_eq!(result.year(), 2025);
    assert_eq!(result.month(), 2);
    assert_eq!(result.day(), 28);
}

#[test]
fn test_adds_months_across_year_boundary() {
    let base = Utc.with_ymd_and_hms(2025, 12, 15, 0, 0, 0).unwrap();
    let result = umt_add_duration(&base, 2, DurationUnit::Month);
    assert_eq!(result.year(), 2026);
    assert_eq!(result.month(), 2);
    assert_eq!(result.day(), 15);
}

#[test]
fn test_subtracts_via_negative_amount() {
    let base = Utc.with_ymd_and_hms(2025, 3, 15, 0, 0, 0).unwrap();
    let result = umt_add_duration(&base, -1, DurationUnit::Month);
    assert_eq!(result.month(), 2);
}

#[test]
fn test_adds_years_preserving_month_and_day() {
    let base = Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap();
    let result = umt_add_duration(&base, 1, DurationUnit::Y);
    assert_eq!(result.year(), 2025);
    assert_eq!(result.month(), 2);
    assert_eq!(result.day(), 28);
}

#[test]
fn test_adds_years_across_leap_year_boundary() {
    let base = Utc.with_ymd_and_hms(2024, 2, 29, 0, 0, 0).unwrap();
    let result = umt_add_duration(&base, 4, DurationUnit::Y);
    assert_eq!(result.year(), 2028);
    assert_eq!(result.month(), 2);
    assert_eq!(result.day(), 29);
}

#[test]
fn test_does_not_mutate_input() {
    let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let snapshot = base.timestamp_millis();
    let _ = umt_add_duration(&base, 5, DurationUnit::Month);
    assert_eq!(base.timestamp_millis(), snapshot);
}
