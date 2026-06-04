//! Tests for the is_business_day module.

use chrono::{TimeZone, Utc};
use umt_rust::date::umt_is_business_day;

#[test]
fn test_returns_true_for_monday_no_holidays() {
    let monday = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
    assert!(umt_is_business_day(&monday, &[]));
}

#[test]
fn test_returns_false_for_saturday() {
    let saturday = Utc.with_ymd_and_hms(2025, 4, 19, 0, 0, 0).unwrap();
    assert!(!umt_is_business_day(&saturday, &[]));
}

#[test]
fn test_returns_false_for_sunday() {
    let sunday = Utc.with_ymd_and_hms(2025, 4, 20, 0, 0, 0).unwrap();
    assert!(!umt_is_business_day(&sunday, &[]));
}

#[test]
fn test_returns_false_for_weekday_listed_as_holiday() {
    let date = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
    let holidays = [Utc.with_ymd_and_hms(2025, 4, 21, 12, 0, 0).unwrap()];
    assert!(!umt_is_business_day(&date, &holidays));
}

#[test]
fn test_returns_true_for_weekday_not_in_holiday_list() {
    let date = Utc.with_ymd_and_hms(2025, 4, 22, 0, 0, 0).unwrap();
    let holidays = [Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap()];
    assert!(umt_is_business_day(&date, &holidays));
}
