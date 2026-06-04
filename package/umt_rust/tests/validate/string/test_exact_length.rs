//! Tests for exact length string validation

use umt_rust::validate::string::{umt_exact_length, umt_validate_string};

#[test]
fn test_exact_length_with_message() {
    let length_validator = umt_exact_length(3, Some("Length must be 3".to_string()));

    let result_invalid = umt_validate_string("abcd", &[length_validator.clone()], None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "Length must be 3");

    let result_valid = umt_validate_string("abc", &[length_validator.clone()], None);
    assert!(result_valid.validate);

    let result_short = umt_validate_string("ab", &[length_validator], None);
    assert!(!result_short.validate);
}

#[test]
fn test_exact_length_without_message() {
    let length_validator = umt_exact_length(3, None);

    let result_valid = umt_validate_string("abc", &[length_validator.clone()], None);
    assert_eq!(result_valid.message, "");
    assert!(result_valid.validate);

    let result_invalid = umt_validate_string("abcd", &[length_validator], None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "");
}

#[test]
fn test_exact_length_zero() {
    let length_validator = umt_exact_length(0, None);

    let result_valid = umt_validate_string("", &[length_validator.clone()], None);
    assert!(result_valid.validate);

    let result_invalid = umt_validate_string("a", &[length_validator], None);
    assert!(!result_invalid.validate);
}
