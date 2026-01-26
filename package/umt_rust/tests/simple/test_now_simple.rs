//! Now Simple integration tests.
//!
//! Ported from TypeScript: nowSimple.test.ts
//!
//! Note: Unlike TypeScript tests that use fake timers, these tests verify
//! the function returns valid DateTime values.

use umt_rust::simple::date::{umt_now_simple, umt_now_simple_jst};

// ============================================================================
// Basic functionality tests
// ============================================================================

#[test]
fn test_should_return_current_date_with_default_timezone() {
    let result = umt_now_simple(9);

    // Verify it's a valid DateTime
    assert!(result.year >= 2024, "Year should be 2024 or later");
    assert!(result.month >= 1 && result.month <= 12);
    assert!(result.day >= 1 && result.day <= 31);
    assert!(result.hour < 24);
    assert!(result.minute < 60);
    assert!(result.second < 60);
    assert!(result.day_of_week < 7);
}

// ============================================================================
// Valid HoursTypeInt values tests
// ============================================================================

#[test]
fn test_should_handle_timezone_0() {
    let result = umt_now_simple(0);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

#[test]
fn test_should_handle_timezone_5() {
    let result = umt_now_simple(5);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

#[test]
fn test_should_handle_timezone_12() {
    let result = umt_now_simple(12);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

#[test]
fn test_should_handle_timezone_23() {
    let result = umt_now_simple(23);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

// ============================================================================
// All valid hours tests
// ============================================================================

#[test]
fn test_should_handle_all_valid_hours_type_int_values() {
    let valid_hours: Vec<i32> = (0..=23).collect();

    for hour in valid_hours {
        let result = umt_now_simple(hour);
        assert!(result.year >= 2024, "Invalid year for timezone {}", hour);
        assert!(result.hour < 24, "Invalid hour for timezone {}", hour);
    }
}

// ============================================================================
// Edge case tests
// ============================================================================

#[test]
fn test_should_handle_midnight_timezone() {
    let result = umt_now_simple(0);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

#[test]
fn test_should_handle_noon_timezone() {
    let result = umt_now_simple(12);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

#[test]
fn test_should_handle_last_hour_timezone() {
    let result = umt_now_simple(23);
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

// ============================================================================
// Timezone comparison tests
// ============================================================================

#[test]
fn test_different_timezones_produce_different_hours() {
    let utc = umt_now_simple(0);
    let jst = umt_now_simple(9);

    // JST should be 9 hours ahead
    // The timestamps should differ by 9 hours worth of milliseconds
    let diff = jst.timestamp_ms - utc.timestamp_ms;
    let expected_diff = 9 * 3600 * 1000; // 9 hours in milliseconds

    assert_eq!(diff, expected_diff, "Timezone offset difference incorrect");
}

// ============================================================================
// JST convenience function tests
// ============================================================================

#[test]
fn test_jst_convenience_function() {
    let now_jst = umt_now_simple_jst();
    let now_manual = umt_now_simple(9);

    // Should be very close in time (within a second)
    assert!(
        (now_jst.timestamp_ms - now_manual.timestamp_ms).abs() < 1000,
        "JST convenience function should match manual timezone 9"
    );
}

// ============================================================================
// DateTime properties tests
// ============================================================================

#[test]
fn test_get_utc_hours() {
    let result = umt_now_simple(9);
    let hours = result.get_utc_hours();
    assert!(hours < 24);
}

#[test]
fn test_get_full_year() {
    let result = umt_now_simple(9);
    let year = result.get_full_year();
    assert!(year >= 2024);
}

#[test]
fn test_get_month_is_zero_indexed() {
    let result = umt_now_simple(9);
    let month = result.get_month();
    // JavaScript-style 0-indexed month (0-11)
    assert!(month < 12, "Month should be 0-11, got {}", month);
}

#[test]
fn test_get_date() {
    let result = umt_now_simple(9);
    let day = result.get_date();
    assert!(day >= 1 && day <= 31, "Day should be 1-31, got {}", day);
}

#[test]
fn test_get_day() {
    let result = umt_now_simple(9);
    let day_of_week = result.get_day();
    assert!(
        day_of_week < 7,
        "Day of week should be 0-6, got {}",
        day_of_week
    );
}

// ============================================================================
// Negative timezone tests
// ============================================================================

#[test]
fn test_should_handle_negative_timezone() {
    let result = umt_now_simple(-5); // EST-like
    assert!(result.year >= 2024);
    assert!(result.hour < 24);
}

#[test]
fn test_negative_timezone_adjusts_correctly() {
    let utc = umt_now_simple(0);
    let negative = umt_now_simple(-5);

    // Negative timezone should be behind UTC
    let diff = negative.timestamp_ms - utc.timestamp_ms;
    let expected_diff = -5 * 3600 * 1000; // -5 hours in milliseconds

    assert_eq!(diff, expected_diff, "Negative timezone offset incorrect");
}

use umt_rust::simple::date::*;

#[test]
fn test_datetime_from_known_timestamp() {
    // 2024-01-01T12:00:00Z
    let timestamp_ms = 1704110400000_i64;
    let dt = DateTime::from_timestamp(timestamp_ms, 0);

    assert_eq!(dt.year, 2024);
    assert_eq!(dt.month, 1);
    assert_eq!(dt.day, 1);
    assert_eq!(dt.hour, 12);
    assert_eq!(dt.minute, 0);
    assert_eq!(dt.second, 0);
}

#[test]
fn test_datetime_with_timezone_offset() {
    // 2024-01-01T12:00:00Z with UTC+9 should be 21:00
    let timestamp_ms = 1704110400000_i64;
    let dt = DateTime::from_timestamp(timestamp_ms, 9);

    assert_eq!(dt.hour, 21);
}

#[test]
fn test_day_of_week_calculation() {
    // Known dates and their days of week
    // 2022-01-02 is Sunday (0)
    let dt1 = umt_new_date(2022, 1, 2);
    assert_eq!(dt1.day_of_week, 0);

    // 2022-01-03 is Monday (1)
    let dt2 = umt_new_date(2022, 1, 3);
    assert_eq!(dt2.day_of_week, 1);

    // 2022-01-08 is Saturday (6)
    let dt3 = umt_new_date(2022, 1, 8);
    assert_eq!(dt3.day_of_week, 6);
}

#[test]
fn test_get_methods() {
    let dt = umt_new_date(2024, 6, 15);
    assert_eq!(dt.get_full_year(), 2024);
    assert_eq!(dt.get_month(), 5); // JavaScript-style (0-indexed)
    assert_eq!(dt.get_date(), 15);
}

#[test]
fn test_leap_year() {
    // 2024-02-29 should be valid (2024 is a leap year)
    let dt = umt_new_date(2024, 2, 29);
    assert_eq!(dt.year, 2024);
    assert_eq!(dt.month, 2);
    assert_eq!(dt.day, 29);
}

#[test]
fn test_new_date() {
    let dt = umt_new_date(2022, 1, 2);
    assert_eq!(dt.year, 2022);
    assert_eq!(dt.month, 1);
    assert_eq!(dt.day, 2);
    assert_eq!(dt.day_of_week, 0); // Sunday
}

#[test]
fn test_now_simple_different_timezones() {
    let now_utc = umt_now_simple(0);
    let now_jst = umt_now_simple(9);

    // JST should be 9 hours ahead of UTC
    // Note: This is a simplified test; actual time difference depends on day boundaries
    assert!(now_jst.timestamp_ms > now_utc.timestamp_ms);
}

#[test]
fn test_now_simple_returns_datetime() {
    let now = umt_now_simple(9);
    assert!(now.year >= 2024);
    assert!(now.month >= 1 && now.month <= 12);
    assert!(now.day >= 1 && now.day <= 31);
    assert!(now.hour < 24);
    assert!(now.minute < 60);
    assert!(now.second < 60);
    assert!(now.day_of_week < 7);
}

#[test]
fn test_valid_hours_type_int_values() {
    let valid_hours: Vec<i32> = (0..=23).collect();
    for hour in valid_hours {
        let now = umt_now_simple(hour);
        assert!(now.year >= 2024);
    }
}
