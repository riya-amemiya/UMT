//! Day of Week Simple integration tests.
//!
//! Ported from TypeScript: dayOfWeekSimple.test.ts

use umt_rust::simple::date::{
    DateProperties, umt_day_of_week_simple, umt_day_of_week_simple_datetime,
    umt_day_of_week_simple_str, umt_new_date,
};

// ============================================================================
// String format tests (hyphen delimiter)
// ============================================================================

#[test]
fn test_should_return_0_for_sunday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-02", 9), 0);
}

#[test]
fn test_should_return_1_for_monday_slash() {
    assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
}

#[test]
fn test_should_return_2_for_tuesday_colon() {
    assert_eq!(umt_day_of_week_simple_str("2022:01:04", 9), 2);
}

#[test]
fn test_should_return_3_for_wednesday_properties() {
    let props = DateProperties::new(Some(2022), Some(1), Some(5));
    assert_eq!(umt_day_of_week_simple(Some(props), 9), 3);
}

#[test]
fn test_should_return_4_for_thursday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-06", 9), 4);
}

#[test]
fn test_should_return_5_for_friday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-07", 9), 5);
}

#[test]
fn test_should_return_6_for_saturday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-08", 9), 6);
}

// ============================================================================
// DateTime object tests
// ============================================================================

#[test]
fn test_should_return_0_for_sunday_datetime() {
    let date = umt_new_date(2022, 1, 2);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 0);
}

#[test]
fn test_should_return_1_for_monday_datetime() {
    let date = umt_new_date(2022, 1, 3);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 1);
}

#[test]
fn test_should_return_2_for_tuesday_datetime() {
    let date = umt_new_date(2022, 1, 4);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 2);
}

#[test]
fn test_should_return_3_for_wednesday_datetime() {
    let date = umt_new_date(2022, 1, 5);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 3);
}

#[test]
fn test_should_return_4_for_thursday_datetime() {
    let date = umt_new_date(2022, 1, 6);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 4);
}

#[test]
fn test_should_return_5_for_friday_datetime() {
    let date = umt_new_date(2022, 1, 7);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 5);
}

#[test]
fn test_should_return_6_for_saturday_datetime() {
    let date = umt_new_date(2022, 1, 8);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 6);
}

// ============================================================================
// Additional tests for known dates
// ============================================================================

#[test]
fn test_known_date_2024_01_01_is_monday() {
    assert_eq!(umt_day_of_week_simple_str("2024-01-01", 9), 1);
}

#[test]
fn test_known_date_2024_12_25_is_wednesday() {
    assert_eq!(umt_day_of_week_simple_str("2024-12-25", 9), 3);
}

#[test]
fn test_known_date_2000_01_01_was_saturday() {
    assert_eq!(umt_day_of_week_simple_str("2000-01-01", 9), 6);
}

// ============================================================================
// Default properties test
// ============================================================================

#[test]
fn test_with_default_properties_returns_current_day() {
    let day = umt_day_of_week_simple(None, 9);
    assert!(day < 7, "Day of week should be 0-6, got {}", day);
}

// ============================================================================
// Format variation tests
// ============================================================================

#[test]
fn test_slash_delimiter_sunday() {
    assert_eq!(umt_day_of_week_simple_str("2022/01/02", 9), 0);
}

#[test]
fn test_slash_delimiter_monday() {
    assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
}

#[test]
fn test_colon_delimiter_sunday() {
    assert_eq!(umt_day_of_week_simple_str("2022:01:02", 9), 0);
}

#[test]
fn test_colon_delimiter_monday() {
    assert_eq!(umt_day_of_week_simple_str("2022:01:03", 9), 1);
}

use umt_rust::simple::date::*;

#[test]
fn test_friday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-07", 9), 5);
}

#[test]
fn test_known_dates() {
    // Additional verification with known dates
    // 2024-01-01 is Monday
    assert_eq!(umt_day_of_week_simple_str("2024-01-01", 9), 1);
    // 2024-12-25 is Wednesday
    assert_eq!(umt_day_of_week_simple_str("2024-12-25", 9), 3);
    // 2000-01-01 was Saturday
    assert_eq!(umt_day_of_week_simple_str("2000-01-01", 9), 6);
}

#[test]
fn test_monday() {
    assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
}

#[test]
fn test_partial_properties() {
    // Test with partial properties - only year specified
    let props = DateProperties::new(Some(2022), None, None);
    let day = umt_day_of_week_simple(Some(props), 9);
    // Should use 2022 with current month and day
    assert!(day < 7);
}

#[test]
fn test_saturday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-08", 9), 6);
}

#[test]
fn test_sunday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-02", 9), 0);
}

#[test]
fn test_thursday() {
    assert_eq!(umt_day_of_week_simple_str("2022-01-06", 9), 4);
}

#[test]
fn test_tuesday() {
    assert_eq!(umt_day_of_week_simple_str("2022:01:04", 9), 2);
}

#[test]
fn test_wednesday() {
    let props = DateProperties::new(Some(2022), Some(1), Some(5));
    assert_eq!(umt_day_of_week_simple(Some(props), 9), 3);
}

#[test]
fn test_with_colon_delimiter() {
    assert_eq!(umt_day_of_week_simple_str("2022:01:02", 9), 0);
    assert_eq!(umt_day_of_week_simple_str("2022:01:03", 9), 1);
}

#[test]
fn test_with_datetime() {
    let date = umt_new_date(2022, 1, 2);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 0); // Sunday
}

#[test]
fn test_with_datetime_monday() {
    let date = umt_new_date(2022, 1, 3);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 1); // Monday
}

#[test]
fn test_with_datetime_saturday() {
    let date = umt_new_date(2022, 1, 8);
    assert_eq!(umt_day_of_week_simple_datetime(&date, 9), 6); // Saturday
}

#[test]
fn test_with_default_properties() {
    // When called with None, should return current day of week
    let day = umt_day_of_week_simple(None, 9);
    assert!(day < 7);
}

#[test]
fn test_with_slash_delimiter() {
    assert_eq!(umt_day_of_week_simple_str("2022/01/02", 9), 0);
    assert_eq!(umt_day_of_week_simple_str("2022/01/03", 9), 1);
}
