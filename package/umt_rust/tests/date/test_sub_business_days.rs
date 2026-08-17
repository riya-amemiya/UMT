//! Tests for the sub_business_days module.

use chrono::{Datelike, TimeZone, Timelike, Utc};
use umt_rust::date::{umt_add_business_days, umt_sub_business_days};

#[test]
fn test_subtracts_one_business_day_from_monday_to_friday() {
    let monday = Utc.with_ymd_and_hms(2025, 4, 21, 9, 30, 0).unwrap();
    let result = umt_sub_business_days(&monday, 1, &[]);
    assert_eq!(result.day(), 18);
    assert_eq!(result.hour(), 9);
    assert_eq!(result.minute(), 30);
}

#[test]
fn test_matches_add_business_days_with_negated_amount() {
    let date = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
    let holiday = Utc.with_ymd_and_hms(2025, 4, 18, 0, 0, 0).unwrap();
    assert_eq!(
        umt_sub_business_days(&date, 2, &[holiday]),
        umt_add_business_days(&date, -2, &[holiday])
    );
}

#[test]
fn test_returns_a_clone_when_amount_is_zero() {
    let date = Utc.with_ymd_and_hms(2025, 4, 21, 8, 0, 0).unwrap();
    let result = umt_sub_business_days(&date, 0, &[]);
    assert_eq!(result, date);
}
