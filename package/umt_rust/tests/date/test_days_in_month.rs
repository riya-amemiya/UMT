//! Tests for the days_in_month module.

use chrono::{DateTime, TimeZone, Utc};
use umt_rust::date::umt_days_in_month;

fn date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap()
}

#[test]
fn test_returns_31_for_31_day_months() {
    for month in [1, 3, 5, 7, 8, 10, 12] {
        assert_eq!(umt_days_in_month(&date(2025, month, 15)), 31);
    }
}

#[test]
fn test_returns_30_for_30_day_months() {
    for month in [4, 6, 9, 11] {
        assert_eq!(umt_days_in_month(&date(2025, month, 15)), 30);
    }
}

#[test]
fn test_returns_28_for_february_in_common_year() {
    assert_eq!(umt_days_in_month(&date(2025, 2, 1)), 28);
    assert_eq!(umt_days_in_month(&date(2100, 2, 1)), 28);
}

#[test]
fn test_returns_29_for_february_in_leap_year() {
    assert_eq!(umt_days_in_month(&date(2024, 2, 1)), 29);
    assert_eq!(umt_days_in_month(&date(2000, 2, 1)), 29);
    assert_eq!(umt_days_in_month(&date(0, 2, 1)), 29);
}

#[test]
fn test_does_not_depend_on_day_or_time() {
    let late = Utc.with_ymd_and_hms(2025, 1, 31, 23, 59, 59).unwrap();
    assert_eq!(umt_days_in_month(&late), 31);
    assert_eq!(umt_days_in_month(&date(2024, 2, 29)), 29);
}
