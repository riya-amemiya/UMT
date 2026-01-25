//! Tests for string length validation

use umt_rust::validate::string::{umt_length, umt_validate_string};

#[test]
fn test_length_with_message() {
    let length_validator = umt_length(3, Some("Length must be 3".to_string()));
    let validators = vec![length_validator.clone()];

    let result_invalid = umt_validate_string("abcd", &validators, None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "Length must be 3");

    let length_validator2 = umt_length(3, Some("Length must be 3".to_string()));
    let validators2 = vec![length_validator2];
    let result_valid = umt_validate_string("abc", &validators2, None);
    assert!(result_valid.validate);

    let length_validator3 = umt_length(3, Some("Length must be 3".to_string()));
    let validators3 = vec![length_validator3];
    let result_short = umt_validate_string("ab", &validators3, None);
    assert!(!result_short.validate);
}

#[test]
fn test_length_without_message() {
    let length_validator = umt_length(3, None);
    let validators = vec![length_validator.clone()];

    let result_valid = umt_validate_string("abc", &validators, None);
    assert!(result_valid.validate);

    let length_validator2 = umt_length(3, None);
    let validators2 = vec![length_validator2];
    let result_invalid = umt_validate_string("abcd", &validators2, None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "");
}

#[test]
fn test_length_zero() {
    let length_validator = umt_length(0, None);
    let validators = vec![length_validator.clone()];

    let result_valid = umt_validate_string("", &validators, None);
    assert!(result_valid.validate);

    let length_validator2 = umt_length(0, None);
    let validators2 = vec![length_validator2];
    let result_invalid = umt_validate_string("a", &validators2, None);
    assert!(!result_invalid.validate);
}

#[test]
fn test_length_exact_match() {
    let length_validator = umt_length(5, None);
    let validators = vec![length_validator];

    let result = umt_validate_string("hello", &validators, None);
    assert!(result.validate);
}
