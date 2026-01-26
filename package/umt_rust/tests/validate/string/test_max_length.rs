//! Tests for string max length validation

use umt_rust::validate::string::{umt_max_length, umt_validate_string};

#[test]
fn test_max_length_with_message() {
    let max_length_validator = umt_max_length(3, Some("Maximum length is 3".to_string()));
    let validators = vec![max_length_validator.clone()];

    let result_invalid = umt_validate_string("abcd", &validators, None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "Maximum length is 3");

    let max_length_validator2 = umt_max_length(3, Some("Maximum length is 3".to_string()));
    let validators2 = vec![max_length_validator2];
    let result_exact = umt_validate_string("abc", &validators2, None);
    assert!(result_exact.validate);

    let max_length_validator3 = umt_max_length(3, Some("Maximum length is 3".to_string()));
    let validators3 = vec![max_length_validator3];
    let result_short = umt_validate_string("ab", &validators3, None);
    assert!(result_short.validate);
}

#[test]
fn test_max_length_without_message() {
    let max_length_validator = umt_max_length(3, None);
    let validators = vec![max_length_validator.clone()];

    let result_valid = umt_validate_string("abc", &validators, None);
    assert!(result_valid.validate);

    let max_length_validator2 = umt_max_length(3, None);
    let validators2 = vec![max_length_validator2];
    let result_invalid = umt_validate_string("abcd", &validators2, None);
    assert!(!result_invalid.validate);
    assert_eq!(result_invalid.message, "");
}

#[test]
fn test_max_length_empty_string() {
    let max_length_validator = umt_max_length(3, None);
    let validators = vec![max_length_validator];

    let result = umt_validate_string("", &validators, None);
    assert!(result.validate);
}

#[test]
fn test_max_length_zero() {
    let max_length_validator = umt_max_length(0, None);
    let validators = vec![max_length_validator.clone()];

    let result_valid = umt_validate_string("", &validators, None);
    assert!(result_valid.validate);

    let max_length_validator2 = umt_max_length(0, None);
    let validators2 = vec![max_length_validator2];
    let result_invalid = umt_validate_string("a", &validators2, None);
    assert!(!result_invalid.validate);
}
