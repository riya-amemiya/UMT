//! Tests for the date_range module.

use chrono::{Datelike, TimeZone, Utc};
use umt_rust::date::umt_date_range;

#[test]
fn test_date_range_generates_array_of_dates() {
    let start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 3, 0, 0, 0).unwrap();
    let dates = umt_date_range(start, end);

    assert_eq!(dates.len(), 3);
    assert_eq!(dates[0].format("%Y-%m-%d").to_string(), "2025-01-01");
    assert_eq!(dates[1].format("%Y-%m-%d").to_string(), "2025-01-02");
    assert_eq!(dates[2].format("%Y-%m-%d").to_string(), "2025-01-03");
}

#[test]
fn test_date_range_single_day() {
    let date = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let dates = umt_date_range(date, date);

    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0], date);
}

#[test]
fn test_date_range_creates_new_instances() {
    let start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let dates = umt_date_range(start, end);

    // In Rust, DateTime is Copy, but we can verify the date is correct
    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].day(), 1);
}

#[test]
fn test_date_range_month_and_year_transitions() {
    let start = Utc.with_ymd_and_hms(2024, 12, 30, 0, 0, 0).unwrap();
    let end = Utc.with_ymd_and_hms(2025, 1, 2, 0, 0, 0).unwrap();
    let dates = umt_date_range(start, end);

    assert_eq!(dates.len(), 4);
    assert_eq!(dates[0].format("%Y-%m-%d").to_string(), "2024-12-30");
    assert_eq!(dates[1].format("%Y-%m-%d").to_string(), "2024-12-31");
    assert_eq!(dates[2].format("%Y-%m-%d").to_string(), "2025-01-01");
    assert_eq!(dates[3].format("%Y-%m-%d").to_string(), "2025-01-02");
}
