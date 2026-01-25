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
