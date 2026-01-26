//! Birthday Simple integration tests.
//!
//! Ported from TypeScript: birthdaySimple.test.ts
//!
//! Note: Unlike TypeScript tests that use fake timers, these tests use actual
//! system time and verify results are within expected ranges.

use umt_rust::simple::date::{
    BirthdayProperties, umt_birthday_simple, umt_birthday_simple_datetime,
    umt_birthday_simple_props, umt_birthday_simple_str, umt_new_date, umt_now_simple,
};

// ============================================================================
// String format with hyphen delimiter
// ============================================================================

#[test]
fn test_basic_age_calculation_hyphen() {
    let age = umt_birthday_simple_str("2000-01-01", 9);
    // Age should be between 24 and 30 depending on current date
    assert!(age >= 24 && age <= 30, "Age {} out of expected range", age);
}

#[test]
fn test_very_old_birthday() {
    let age = umt_birthday_simple_str("1900-01-01", 9);
    // Should be around 124-126
    assert!(
        age >= 124 && age <= 130,
        "Very old age {} out of expected range",
        age
    );
}

#[test]
fn test_future_birthday_returns_zero() {
    let age = umt_birthday_simple_str("2100-01-01", 9);
    assert_eq!(age, 0);
}

#[test]
fn test_leap_year_birthday_hyphen() {
    let age = umt_birthday_simple_str("2000-02-29", 9);
    // Leap year should still calculate correctly
    assert!(age >= 24 && age <= 30, "Leap year age {} out of range", age);
}

#[test]
fn test_end_of_year_birthday() {
    let age = umt_birthday_simple_str("2000-12-31", 9);
    // Should be around 23-25 depending on current date
    assert!(
        age >= 23 && age <= 30,
        "End of year age {} out of range",
        age
    );
}

// ============================================================================
// String format with colon delimiter
// ============================================================================

#[test]
fn test_basic_age_calculation_colon() {
    let age = umt_birthday_simple_str("2000:01:01", 9);
    assert!(
        age >= 24 && age <= 30,
        "Colon format age {} out of range",
        age
    );
}

#[test]
fn test_various_patterns_colon() {
    let age1 = umt_birthday_simple_str("2000:02:29", 9);
    assert!(age1 >= 24 && age1 <= 30);

    let age2 = umt_birthday_simple_str("2000:12:01", 9);
    assert!(age2 >= 23 && age2 <= 30);
}

// ============================================================================
// String format with slash delimiter
// ============================================================================

#[test]
fn test_basic_age_calculation_slash() {
    let age = umt_birthday_simple_str("2000/01/01", 9);
    assert!(
        age >= 24 && age <= 30,
        "Slash format age {} out of range",
        age
    );
}

#[test]
fn test_various_patterns_slash() {
    let age1 = umt_birthday_simple_str("2000/02/29", 9);
    assert!(age1 >= 24 && age1 <= 30);

    let age2 = umt_birthday_simple_str("2000/11/30", 9);
    assert!(age2 >= 23 && age2 <= 30);
}

// ============================================================================
// DateTime object format
// ============================================================================

#[test]
fn test_basic_age_calculation_datetime() {
    let birthday = umt_new_date(2000, 1, 1);
    let age = umt_birthday_simple_datetime(&birthday, 9);
    assert!(age >= 24 && age <= 30, "DateTime age {} out of range", age);
}

#[test]
fn test_leap_year_datetime() {
    let birthday = umt_new_date(2000, 2, 29);
    let age = umt_birthday_simple_datetime(&birthday, 9);
    assert!(age >= 24 && age <= 30);
}

// ============================================================================
// Object format (BirthdayProperties)
// ============================================================================

#[test]
fn test_basic_age_calculation_properties() {
    let props = BirthdayProperties::new(2000, 1, 1);
    let age = umt_birthday_simple_props(&props, 9);
    assert!(age >= 24 && age <= 30, "Props age {} out of range", age);
}

#[test]
fn test_month_and_day_edge_cases_properties() {
    let props1 = BirthdayProperties::new(2000, 2, 29);
    let age1 = umt_birthday_simple_props(&props1, 9);
    assert!(age1 >= 24 && age1 <= 30);

    let props2 = BirthdayProperties::new(2000, 12, 31);
    let age2 = umt_birthday_simple_props(&props2, 9);
    assert!(age2 >= 23 && age2 <= 30);
}

// ============================================================================
// Timezone difference tests
// ============================================================================

#[test]
fn test_different_timezone_differences() {
    let age_utc = umt_birthday_simple_str("2000-01-01", 0);
    let age_est = umt_birthday_simple_str("2000-01-01", 5);
    let age_cst = umt_birthday_simple_str("2000-01-01", 8);
    let age_jst = umt_birthday_simple_str("2000-01-01", 9);

    // Ages should be the same or differ by at most 1 due to timezone boundary
    assert!((age_utc - age_jst).abs() <= 1);
    assert!((age_est - age_jst).abs() <= 1);
    assert!((age_cst - age_jst).abs() <= 1);
}

#[test]
fn test_timezone_for_each_format() {
    let age1 = umt_birthday_simple_str("2000:01:01", 0);
    let age2 = umt_birthday_simple_str("2000/01/01", 8);
    let age3 = umt_birthday_simple_datetime(&umt_new_date(2000, 1, 1), 5);
    let age4 = umt_birthday_simple_props(&BirthdayProperties::new(2000, 1, 1), 3);

    // All should be approximately the same
    assert!((age1 - age2).abs() <= 1);
    assert!((age2 - age3).abs() <= 1);
    assert!((age3 - age4).abs() <= 1);
}

// ============================================================================
// Boundary value tests
// ============================================================================

#[test]
fn test_very_old_birthdays() {
    let age1 = umt_birthday_simple_str("1900-01-01", 9);
    assert!(age1 >= 124 && age1 <= 130);

    let age2 = umt_birthday_simple_str("1950-06-15", 9);
    assert!(age2 >= 73 && age2 <= 80);
}

#[test]
fn test_recent_birthdays() {
    let now = umt_now_simple(9);
    let birth_year = now.year - 1;
    let date_str = format!("{}-01-01", birth_year);
    let age = umt_birthday_simple_str(&date_str, 9);
    // Should be 0, 1, or 2 depending on current date
    assert!(age <= 2, "Recent birthday age {} too high", age);
}

#[test]
fn test_month_and_day_boundaries() {
    let age1 = umt_birthday_simple_str("2000-01-31", 9);
    assert!(age1 >= 24 && age1 <= 30);

    let age2 = umt_birthday_simple_str("2000-03-31", 9);
    assert!(age2 >= 24 && age2 <= 30);

    let age3 = umt_birthday_simple_str("2000-04-30", 9);
    assert!(age3 >= 24 && age3 <= 30);

    let age4 = umt_birthday_simple_str("2000-05-31", 9);
    assert!(age4 >= 24 && age4 <= 30);
}

// ============================================================================
// Convenience function tests
// ============================================================================

#[test]
fn test_convenience_function() {
    let age = umt_birthday_simple("2000-01-01");
    assert!(age >= 24 && age <= 30);
}

// ============================================================================
// Age is always non-negative
// ============================================================================

#[test]
fn test_age_is_non_negative() {
    let dates = vec!["2100-01-01", "2050-06-15", "1900-01-01", "2000-01-01"];

    for date in dates {
        let age = umt_birthday_simple_str(date, 9);
        assert!(age >= 0, "Age should be non-negative for date {}", date);
    }
}

use umt_rust::simple::date::*;

#[test]
fn test_different_timezones() {
    let age_utc = umt_birthday_simple_str("2000-01-01", 0);
    let age_jst = umt_birthday_simple_str("2000-01-01", 9);
    // Ages should be the same or differ by at most 1 (due to timezone boundary)
    assert!((age_utc - age_jst).abs() <= 1);
}

#[test]
fn test_future_birthday() {
    // Far future birthday should return 0
    let age = umt_birthday_simple_str("2100-01-01", 9);
    assert_eq!(age, 0);
}

#[test]
fn test_invalid_date_string() {
    // Invalid format should return 0
    let age = umt_birthday_simple_str("invalid", 9);
    assert_eq!(age, 0);
}

#[test]
fn test_leap_year() {
    let age = umt_birthday_simple_str("2000-02-29", 9);
    assert!(age >= 24 && age <= 30);
}

#[test]
fn test_month_boundary() {
    let age = umt_birthday_simple_str("2000-12-31", 9);
    assert!(age >= 23 && age <= 30);
}

#[test]
fn test_recent_birthday() {
    // Someone born 1 year ago (approximately)
    let now = umt_now_simple(9);
    let birth_year = now.year - 1;
    let date_str = format!("{}-01-01", birth_year);
    let age = umt_birthday_simple_str(&date_str, 9);
    // Should be 0 or 1 depending on whether birthday has passed
    assert!(age <= 2);
}

#[test]
fn test_with_datetime() {
    let birthday = umt_new_date(2000, 1, 1);
    let age = umt_birthday_simple_datetime(&birthday, 9);
    assert!(age >= 24 && age <= 30);
}

#[test]
fn test_with_properties() {
    let props = BirthdayProperties::new(2000, 1, 1);
    let age = umt_birthday_simple_props(&props, 9);
    assert!(age >= 24 && age <= 30);
}
