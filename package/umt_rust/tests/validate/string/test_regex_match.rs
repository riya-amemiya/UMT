//! Tests for regex match validation

use regex::Regex;
use umt_rust::validate::string::umt_regex_match;

#[test]
fn test_regex_match_validates_matching_patterns() {
    let pattern = Regex::new(r"^[a-z]+$").unwrap();
    let validator = umt_regex_match(pattern, None);

    assert!((validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"ABC".to_string()));
    assert!(!(validator.validate)(&"123".to_string()));
    assert!(!(validator.validate)(&"abc123".to_string()));
}

#[test]
fn test_regex_match_validates_numeric_patterns() {
    let pattern = Regex::new(r"^[0-9]+$").unwrap();
    let validator = umt_regex_match(pattern, None);

    assert!((validator.validate)(&"123".to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"123abc".to_string()));
    assert!(!(validator.validate)(&"".to_string()));
}

#[test]
fn test_regex_match_handles_custom_error_messages() {
    let pattern = Regex::new(r"^[A-Z]+$").unwrap();
    let message = "Only uppercase letters are allowed";
    let validator = umt_regex_match(pattern, Some(message.to_string()));

    assert!((validator.validate)(&"ABC".to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
    assert_eq!(validator.message, Some(message.to_string()));
}

#[test]
fn test_regex_match_email_pattern() {
    let pattern = Regex::new(r"^[\w.+-]+@[\w-]+\.[\w.-]+$").unwrap();
    let validator = umt_regex_match(pattern, None);

    assert!((validator.validate)(&"test@example.com".to_string()));
    assert!((validator.validate)(
        &"user.name+tag@example.co.uk".to_string()
    ));
    assert!(!(validator.validate)(&"invalid-email".to_string()));
}

#[test]
fn test_regex_match_alphanumeric_pattern() {
    let pattern = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    let validator = umt_regex_match(pattern, None);

    assert!((validator.validate)(&"abc123".to_string()));
    assert!((validator.validate)(&"ABC123".to_string()));
    assert!(!(validator.validate)(&"abc-123".to_string()));
    assert!(!(validator.validate)(&"abc 123".to_string()));
}

#[test]
fn test_regex_match_empty_string() {
    let pattern = Regex::new(r"^[a-z]*$").unwrap();
    let validator = umt_regex_match(pattern, None);

    assert!((validator.validate)(&"".to_string()));
}

#[test]
fn test_regex_match_special_characters() {
    let pattern = Regex::new(r"^[\w!@#$%^&*]+$").unwrap();
    let validator = umt_regex_match(pattern, None);

    assert!((validator.validate)(&"test!@#".to_string()));
    assert!((validator.validate)(&"abc123".to_string()));
}
