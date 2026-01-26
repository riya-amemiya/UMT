use regex::Regex;
use umt_rust::validate::string::*;

#[test]
fn test_validate_string() {
    let result = umt_validate_string("hello", &[], None);
    assert!(result.validate);
}

#[test]
fn test_length() {
    let validator = umt_length(5, None);
    assert!((validator.validate)(&"hello".to_string()));
    assert!(!(validator.validate)(&"hi".to_string()));
}

#[test]
fn test_min_length() {
    let validator = umt_min_length(3, None);
    assert!((validator.validate)(&"hello".to_string()));
    assert!((validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"ab".to_string()));
}

#[test]
fn test_max_length() {
    let validator = umt_max_length(5, None);
    assert!((validator.validate)(&"hello".to_string()));
    assert!((validator.validate)(&"hi".to_string()));
    assert!(!(validator.validate)(&"hello world".to_string()));
}

#[test]
fn test_number_string() {
    let validator = umt_number_string(None);
    assert!((validator.validate)(&"123".to_string()));
    assert!((validator.validate)(&"3.14".to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
}

#[test]
fn test_regex_match() {
    let validator = umt_regex_match(Regex::new(r"^\d+$").unwrap(), None);
    assert!((validator.validate)(&"123".to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
}

#[test]
fn test_uuid() {
    let validator = umt_uuid(None, None);
    assert!((validator.validate)(
        &"550e8400-e29b-41d4-a716-446655440000".to_string()
    ));
    assert!(!(validator.validate)(&"not-a-uuid".to_string()));
}

#[test]
fn test_validate_email() {
    let validator = umt_validate_email_validator(None, None);
    assert!((validator.validate)(&"user@example.com".to_string()));
    assert!(!(validator.validate)(&"invalid-email".to_string()));
}
