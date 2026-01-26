//! Tests for string min length validation

use umt_rust::validate::string::{umt_min_length, umt_validate_string};

#[test]
fn test_min_length_with_message() {
    let min_length_validator = umt_min_length(3, Some("Minimum length is 3".to_string()));
    let validators = vec![min_length_validator.clone()];

    let result_empty = umt_validate_string("", &validators, None);
    assert!(!result_empty.validate);
    assert_eq!(result_empty.message, "Minimum length is 3");

    let min_length_validator2 = umt_min_length(3, Some("Minimum length is 3".to_string()));
    let validators2 = vec![min_length_validator2];
    let result_short = umt_validate_string("ab", &validators2, None);
    assert!(!result_short.validate);

    let min_length_validator3 = umt_min_length(3, Some("Minimum length is 3".to_string()));
    let validators3 = vec![min_length_validator3];
    let result_exact = umt_validate_string("abc", &validators3, None);
    assert!(result_exact.validate);
}

#[test]
fn test_min_length_without_message() {
    let min_length_validator = umt_min_length(3, None);
    let validators = vec![min_length_validator.clone()];

    let result_valid = umt_validate_string("abc", &validators, None);
    assert!(result_valid.validate);

    let min_length_validator2 = umt_min_length(3, None);
    let validators2 = vec![min_length_validator2];
    let result_invalid = umt_validate_string("ab", &validators2, None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "");
}

#[test]
fn test_min_length_zero() {
    let min_length_validator = umt_min_length(0, None);
    let validators = vec![min_length_validator.clone()];

    let result_empty = umt_validate_string("", &validators, None);
    assert!(result_empty.validate);

    let min_length_validator2 = umt_min_length(0, None);
    let validators2 = vec![min_length_validator2];
    let result_any = umt_validate_string("any string", &validators2, None);
    assert!(result_any.validate);
}

#[test]
fn test_min_length_longer_string() {
    let min_length_validator = umt_min_length(3, None);
    let validators = vec![min_length_validator];

    let result = umt_validate_string("hello world", &validators, None);
    assert!(result.validate);
}
