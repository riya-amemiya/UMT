//! Tests for the birthday module.

use umt_rust::date::umt_birthday;

#[test]
fn test_birthday_returns_positive_age() {
    // Test with a date far in the past (someone born in 1990)
    let age = umt_birthday(1990, 1, 1, Some(0));
    assert!(age > 30, "Age should be greater than 30 for someone born in 1990");
}

#[test]
fn test_birthday_future_date_returns_zero() {
    // Test with a date in the future
    let age = umt_birthday(2100, 1, 1, Some(0));
    assert_eq!(age, 0);
}

#[test]
fn test_birthday_invalid_date_returns_zero() {
    // Test with invalid date (Feb 30)
    let age = umt_birthday(2000, 2, 30, Some(0));
    assert_eq!(age, 0);
}

#[test]
fn test_birthday_with_timezone() {
    let age_jst = umt_birthday(1990, 1, 1, Some(9));
    let age_utc = umt_birthday(1990, 1, 1, Some(0));
    // Ages should be the same or differ by at most 1 (depending on date)
    assert!(age_jst.abs_diff(age_utc) <= 1);
}

#[test]
fn test_birthday_default_timezone() {
    let age_default = umt_birthday(1990, 1, 1, None);
    let age_jst = umt_birthday(1990, 1, 1, Some(9));
    assert_eq!(age_default, age_jst);
}
