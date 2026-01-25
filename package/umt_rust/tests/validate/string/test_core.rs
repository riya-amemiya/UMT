//! Tests for string core validation

use umt_rust::validate::string::{umt_max_length, umt_min_length, umt_validate_string};

#[test]
fn test_validate_string_basic() {
    let result = umt_validate_string("", &[], None);
    assert!(result.validate);

    let result_abc = umt_validate_string("abc", &[], None);
    assert!(result_abc.validate);
}

#[test]
fn test_validate_string_with_message() {
    let result = umt_validate_string("", &[], Some("Must be string"));
    assert_eq!(result.message, "Must be string");

    let result_abc = umt_validate_string("abc", &[], None);
    assert!(result_abc.validate);
}

#[test]
fn test_validate_string_with_min_and_max_length() {
    let min_validator = umt_min_length(3, None);
    let max_validator = umt_max_length(5, None);
    let validators = vec![min_validator.clone(), max_validator.clone()];

    let result_short = umt_validate_string("ab", &validators, None);
    assert!(!result_short.validate);

    let min_validator2 = umt_min_length(3, None);
    let max_validator2 = umt_max_length(5, None);
    let validators2 = vec![min_validator2, max_validator2];

    let result_min = umt_validate_string("abc", &validators2, None);
    assert!(result_min.validate);

    let min_validator3 = umt_min_length(3, None);
    let max_validator3 = umt_max_length(5, None);
    let validators3 = vec![min_validator3, max_validator3];

    let result_max = umt_validate_string("abcde", &validators3, None);
    assert!(result_max.validate);

    let min_validator4 = umt_min_length(3, None);
    let max_validator4 = umt_max_length(5, None);
    let validators4 = vec![min_validator4, max_validator4];

    let result_long = umt_validate_string("abcdef", &validators4, None);
    assert!(!result_long.validate);

    let min_validator5 = umt_min_length(3, None);
    let max_validator5 = umt_max_length(5, None);
    let validators5 = vec![min_validator5, max_validator5];

    let result_longer = umt_validate_string("abcdefg", &validators5, None);
    assert!(!result_longer.validate);
}

#[test]
fn test_validate_string_empty_string() {
    let result = umt_validate_string("", &[], None);
    assert!(result.validate);
    assert_eq!(result.type_value, "");
}

#[test]
fn test_validate_string_preserves_value() {
    let result = umt_validate_string("hello world", &[], None);
    assert_eq!(result.type_value, "hello world");
}
