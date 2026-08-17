//! Tests for the add_business_days module.

use chrono::{Datelike, TimeZone, Timelike, Utc};
use umt_rust::date::umt_add_business_days;

#[test]
fn test_adds_one_business_day_from_friday_to_monday() {
    let friday = Utc.with_ymd_and_hms(2025, 4, 18, 9, 30, 0).unwrap();
    let result = umt_add_business_days(&friday, 1, &[]);
    assert_eq!(result.year(), 2025);
    assert_eq!(result.month(), 4);
    assert_eq!(result.day(), 21);
    assert_eq!(result.hour(), 9);
    assert_eq!(result.minute(), 30);
}

#[test]
fn test_adds_one_business_day_from_saturday_to_monday() {
    let saturday = Utc.with_ymd_and_hms(2025, 4, 19, 0, 0, 0).unwrap();
    let result = umt_add_business_days(&saturday, 1, &[]);
    assert_eq!(result.day(), 21);
}

#[test]
fn test_subtracts_one_business_day_from_monday_to_friday() {
    let monday = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
    let result = umt_add_business_days(&monday, -1, &[]);
    assert_eq!(result.day(), 18);
}

#[test]
fn test_returns_a_clone_when_amount_is_zero() {
    let saturday = Utc.with_ymd_and_hms(2025, 4, 19, 12, 0, 0).unwrap();
    let result = umt_add_business_days(&saturday, 0, &[]);
    assert_eq!(result, saturday);
}

#[test]
fn test_skips_holidays_when_walking_forward() {
    let monday = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
    let holiday = Utc.with_ymd_and_hms(2025, 4, 22, 0, 0, 0).unwrap();
    let result = umt_add_business_days(&monday, 1, &[holiday]);
    assert_eq!(result.day(), 23);
}

#[test]
fn test_skips_holidays_when_walking_backward() {
    let wednesday = Utc.with_ymd_and_hms(2025, 4, 23, 0, 0, 0).unwrap();
    let holiday = Utc.with_ymd_and_hms(2025, 4, 22, 0, 0, 0).unwrap();
    let result = umt_add_business_days(&wednesday, -1, &[holiday]);
    assert_eq!(result.day(), 21);
}
