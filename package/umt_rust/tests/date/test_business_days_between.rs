//! Tests for the business_days_between module.

use chrono::{DateTime, TimeZone, Utc};
use umt_rust::date::{umt_add_business_days, umt_business_days_between, umt_is_same_day};

fn date(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, 0)
        .unwrap()
}

#[test]
fn test_returns_zero_for_same_day() {
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 21, 9, 0), &date(2025, 4, 21, 18, 0), &[]),
        0
    );
}

#[test]
fn test_excludes_start_and_includes_end() {
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 21, 0, 0), &date(2025, 4, 22, 0, 0), &[]),
        1
    );
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 21, 0, 0), &date(2025, 4, 25, 0, 0), &[]),
        4
    );
}

#[test]
fn test_skips_weekends() {
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 18, 0, 0), &date(2025, 4, 21, 0, 0), &[]),
        1
    );
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 18, 0, 0), &date(2025, 4, 20, 0, 0), &[]),
        0
    );
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 21, 0, 0), &date(2025, 5, 2, 0, 0), &[]),
        9
    );
}

#[test]
fn test_skips_holidays() {
    assert_eq!(
        umt_business_days_between(
            &date(2025, 4, 21, 0, 0),
            &date(2025, 4, 25, 0, 0),
            &[date(2025, 4, 23, 15, 0)]
        ),
        3
    );
}

#[test]
fn test_returns_negative_count_when_end_before_start() {
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 21, 0, 0), &date(2025, 4, 18, 0, 0), &[]),
        -1
    );
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 25, 0, 0), &date(2025, 4, 21, 0, 0), &[]),
        -4
    );
}

#[test]
fn test_returns_zero_for_weekend_only_backward_range() {
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 20, 0, 0), &date(2025, 4, 19, 0, 0), &[]),
        0
    );
}

#[test]
fn test_ignores_time_of_day() {
    assert_eq!(
        umt_business_days_between(&date(2025, 4, 21, 23, 59), &date(2025, 4, 22, 0, 1), &[]),
        1
    );
}

#[test]
fn test_is_inverse_of_add_business_days() {
    let holidays = [date(2025, 4, 29, 0, 0)];
    let start = date(2025, 4, 18, 9, 30);
    for amount in [-12, -5, -1, 1, 3, 7, 15] {
        let end = umt_add_business_days(&start, amount, &holidays);
        let count = umt_business_days_between(&start, &end, &holidays);
        assert_eq!(count, amount);
        assert!(umt_is_same_day(
            &umt_add_business_days(&start, count, &holidays),
            &end
        ));
    }
}
