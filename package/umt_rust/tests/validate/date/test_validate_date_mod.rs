//! Tests for date core validation

use chrono::{TimeZone, Utc};
use umt_rust::validate::date::{umt_date_validator, umt_validate_date};

#[test]
fn test_validate_date_returns_true_for_valid_date() {
    let value = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let result = umt_validate_date(Some(value), None);
    assert!(result.validate);
    assert_eq!(result.type_value, Some(value));
}

#[test]
fn test_validate_date_returns_false_for_invalid_date() {
    let result = umt_validate_date(None, None);
    assert!(!result.validate);
    assert_eq!(result.type_value, None);
}

#[test]
fn test_validate_date_with_custom_message_on_failure() {
    let custom_message = "This is not a date";
    let result = umt_validate_date(None, Some(custom_message));
    assert!(!result.validate);
    assert_eq!(result.message, custom_message);
}

#[test]
fn test_validate_date_ignores_message_on_success() {
    let value = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let result = umt_validate_date(Some(value), Some("ignored"));
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_date_validator_function() {
    let validator = umt_date_validator(None);
    let value = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
    let result = validator(Some(value));
    assert!(result.validate);
    assert_eq!(result.type_value, Some(value));

    let result_invalid = validator(None);
    assert!(!result_invalid.validate);
}

#[test]
fn test_date_validator_with_message() {
    let validator = umt_date_validator(Some("Custom message".to_string()));
    let result = validator(None);
    assert!(!result.validate);
    assert_eq!(result.message, "Custom message");
}
