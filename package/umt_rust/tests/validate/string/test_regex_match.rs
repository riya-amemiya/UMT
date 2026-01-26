//! Tests for regex match validation

use umt_rust::validate::string::umt_regex_match;

#[test]
fn test_regex_match_validates_matching_patterns() {
    let validator = umt_regex_match(|s: &str| s.chars().all(|c| c.is_ascii_lowercase()), None);

    assert!((validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"ABC".to_string()));
    assert!(!(validator.validate)(&"123".to_string()));
    assert!(!(validator.validate)(&"abc123".to_string()));
}

#[test]
fn test_regex_match_validates_numeric_patterns() {
    let validator = umt_regex_match(|s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()), None);

    assert!((validator.validate)(&"123".to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"123abc".to_string()));
    assert!(!(validator.validate)(&"".to_string()));
}

#[test]
fn test_regex_match_handles_custom_error_messages() {
    let message = "Only uppercase letters are allowed";
    let validator = umt_regex_match(
        |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_uppercase()),
        Some(message.to_string()),
    );

    assert!((validator.validate)(&"ABC".to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
    assert_eq!(validator.message, Some(message.to_string()));
}

#[test]
fn test_regex_match_email_pattern() {
    let validator = umt_regex_match(|s: &str| {
        let parts: Vec<&str> = s.splitn(2, '@').collect();
        if parts.len() != 2 { return false; }
        let local = parts[0];
        let domain = parts[1];
        !local.is_empty() && domain.contains('.') && !domain.starts_with('.') && !domain.ends_with('.')
    }, None);

    assert!((validator.validate)(&"test@example.com".to_string()));
    assert!((validator.validate)(&"user.name+tag@example.co.uk".to_string()));
    assert!(!(validator.validate)(&"invalid-email".to_string()));
}

#[test]
fn test_regex_match_alphanumeric_pattern() {
    let validator = umt_regex_match(|s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric()), None);

    assert!((validator.validate)(&"abc123".to_string()));
    assert!((validator.validate)(&"ABC123".to_string()));
    assert!(!(validator.validate)(&"abc-123".to_string()));
    assert!(!(validator.validate)(&"abc 123".to_string()));
}

#[test]
fn test_regex_match_empty_string() {
    let validator = umt_regex_match(|s: &str| s.chars().all(|c| c.is_ascii_lowercase()), None);

    assert!((validator.validate)(&"".to_string()));
}

#[test]
fn test_regex_match_special_characters() {
    let validator = umt_regex_match(|s: &str| {
        !s.is_empty() && s.chars().all(|c| c.is_ascii_alphanumeric() || "!@#$%^&*_".contains(c))
    }, None);

    assert!((validator.validate)(&"test!@#".to_string()));
    assert!((validator.validate)(&"abc123".to_string()));
}
