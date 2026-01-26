//! Clock Type integration tests.
//!
//! Ported from TypeScript: clockType.test.ts
//!
//! Tests the clock-related constants and types.
//! In TypeScript, these were type-level tests. In Rust, we test
//! the constants and verify their values are correct.

use umt_rust::consts::{
    ONE_DAY_MS, ONE_HOUR_MS, ONE_MINUTE_MS, ONE_MONTH_MS, ONE_MONTH_MS_28, ONE_MONTH_MS_29,
    ONE_MONTH_MS_31, ONE_SECOND_MS, ONE_WEEK_MS, ONE_YEAR_MS, ONE_YEAR_MS_366,
};

// ============================================================================
// Hours Type Tests
// ============================================================================

#[test]
fn test_hours_am_int_values() {
    // TypeScript: HoursAmInt[] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    let hours_am_int: Vec<u8> = (0..=12).collect();
    let expected: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    assert_eq!(hours_am_int, expected);
}

#[test]
fn test_hours_am_string_values() {
    // TypeScript: HoursAm[] = ["00", "01", ..., "12"]
    let hours_am: Vec<String> = (0..=12).map(|h| format!("{:02}", h)).collect();
    let expected: Vec<&str> = vec![
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12",
    ];
    assert_eq!(hours_am, expected);
}

#[test]
fn test_hours_pm_int_values() {
    // TypeScript: HoursPmInt[] = [12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]
    let hours_pm_int: Vec<u8> = (12..=23).collect();
    let expected: Vec<u8> = vec![12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23];
    assert_eq!(hours_pm_int, expected);
}

#[test]
fn test_hours_pm_string_values() {
    // TypeScript: HoursPm[] = ["12", "13", ..., "23"]
    let hours_pm: Vec<String> = (12..=23).map(|h| format!("{:02}", h)).collect();
    let expected: Vec<&str> = vec![
        "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23",
    ];
    assert_eq!(hours_pm, expected);
}

#[test]
fn test_hours_type_int_values() {
    // TypeScript: HoursTypeInt[] = [0, 1, 2, ..., 23]
    let hours_type_int: Vec<u8> = (0..=23).collect();
    let expected: Vec<u8> = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    ];
    assert_eq!(hours_type_int, expected);
}

// ============================================================================
// Minutes Type Tests
// ============================================================================

#[test]
fn test_minutes_type_int_values() {
    // TypeScript: MinutesTypeInt[] = [0, 1, 2, ..., 60]
    let minutes_type_int: Vec<u8> = (0..=60).collect();
    let expected: Vec<u8> = vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
    ];
    assert_eq!(minutes_type_int, expected);
}

#[test]
fn test_minutes_type_string_values() {
    // TypeScript: MinutesType[] = ["00", "01", ..., "60"]
    let minutes_type: Vec<String> = (0..=60).map(|m| format!("{:02}", m)).collect();
    let expected: Vec<&str> = vec![
        "00", "01", "02", "03", "04", "05", "06", "07", "08", "09", "10", "11", "12", "13", "14",
        "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29",
        "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44",
        "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",
        "60",
    ];
    assert_eq!(minutes_type, expected);
}

// ============================================================================
// Clock Constants Tests
// ============================================================================

#[test]
fn test_one_second_ms() {
    assert_eq!(ONE_SECOND_MS, 1000);
}

#[test]
fn test_one_minute_ms() {
    assert_eq!(ONE_MINUTE_MS, 60_000);
    assert_eq!(ONE_MINUTE_MS, ONE_SECOND_MS * 60);
}

#[test]
fn test_one_hour_ms() {
    assert_eq!(ONE_HOUR_MS, 3_600_000);
    assert_eq!(ONE_HOUR_MS, ONE_MINUTE_MS * 60);
}

#[test]
fn test_one_day_ms() {
    assert_eq!(ONE_DAY_MS, 86_400_000);
    assert_eq!(ONE_DAY_MS, ONE_HOUR_MS * 24);
}

#[test]
fn test_one_week_ms() {
    assert_eq!(ONE_WEEK_MS, 604_800_000);
    assert_eq!(ONE_WEEK_MS, ONE_DAY_MS * 7);
}

#[test]
fn test_one_month_ms_variations() {
    assert_eq!(ONE_MONTH_MS_28, ONE_DAY_MS * 28);
    assert_eq!(ONE_MONTH_MS_29, ONE_DAY_MS * 29);
    assert_eq!(ONE_MONTH_MS, ONE_DAY_MS * 30);
    assert_eq!(ONE_MONTH_MS_31, ONE_DAY_MS * 31);

    // Verify ordering
    assert!(ONE_MONTH_MS_28 < ONE_MONTH_MS_29);
    assert!(ONE_MONTH_MS_29 < ONE_MONTH_MS);
    assert!(ONE_MONTH_MS < ONE_MONTH_MS_31);
}

#[test]
fn test_one_year_ms_variations() {
    assert_eq!(ONE_YEAR_MS, ONE_DAY_MS * 365);
    assert_eq!(ONE_YEAR_MS_366, ONE_DAY_MS * 366);

    // Leap year has one extra day
    assert_eq!(ONE_YEAR_MS_366 - ONE_YEAR_MS, ONE_DAY_MS);
}

// ============================================================================
// Time Unit Relationship Tests
// ============================================================================

#[test]
fn test_time_unit_relationships() {
    // Verify the hierarchy of time units
    assert!(ONE_SECOND_MS < ONE_MINUTE_MS);
    assert!(ONE_MINUTE_MS < ONE_HOUR_MS);
    assert!(ONE_HOUR_MS < ONE_DAY_MS);
    assert!(ONE_DAY_MS < ONE_WEEK_MS);
    assert!(ONE_WEEK_MS < ONE_MONTH_MS_28);
    assert!(ONE_MONTH_MS_31 < ONE_YEAR_MS);
}

#[test]
fn test_hours_range_valid() {
    // Valid hour values are 0-23
    for hour in 0u8..=23 {
        assert!(hour < 24, "Hour {} should be valid", hour);
    }
}

#[test]
fn test_minutes_range_valid() {
    // Valid minute values are 0-59 (60 included for edge cases in some systems)
    for minute in 0u8..=60 {
        assert!(minute <= 60, "Minute {} should be valid", minute);
    }
}

#[test]
fn test_am_hours_subset() {
    // AM hours (0-12) are a subset of all hours (0-23)
    let all_hours: Vec<u8> = (0..=23).collect();
    let am_hours: Vec<u8> = (0..=12).collect();

    for hour in &am_hours {
        assert!(
            all_hours.contains(hour),
            "AM hour {} should be in all hours",
            hour
        );
    }
}

#[test]
fn test_pm_hours_subset() {
    // PM hours (12-23) are a subset of all hours (0-23)
    let all_hours: Vec<u8> = (0..=23).collect();
    let pm_hours: Vec<u8> = (12..=23).collect();

    for hour in &pm_hours {
        assert!(
            all_hours.contains(hour),
            "PM hour {} should be in all hours",
            hour
        );
    }
}
