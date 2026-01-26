//! Tests for number string validation

use umt_rust::validate::string::umt_number_string;

#[test]
fn test_number_string_returns_true_for_valid_number_strings() {
    let validator = umt_number_string(None);
    assert!((validator.validate)(&"123".to_string()));
    assert!((validator.validate)(&"0.56".to_string()));
    assert!((validator.validate)(&"3.14".to_string()));
    assert!((validator.validate)(&"-42".to_string()));
}

#[test]
fn test_number_string_returns_false_for_invalid_number_strings() {
    let validator = umt_number_string(None);
    assert!(!(validator.validate)(&"abc".to_string()));
    assert!(!(validator.validate)(&"123abc".to_string()));
}

#[test]
fn test_number_string_empty_string() {
    let validator = umt_number_string(None);
    // In Rust, empty string parsing to f64 fails, so it should be false
    assert!(!(validator.validate)(&"".to_string()));
}

#[test]
fn test_number_string_returns_custom_message() {
    let custom_message = "This is not a valid number string";
    let validator = umt_number_string(Some(custom_message.to_string()));
    assert!(!(validator.validate)(&"abc".to_string()));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_number_string_scientific_notation() {
    let validator = umt_number_string(None);
    assert!((validator.validate)(&"1e10".to_string()));
    assert!((validator.validate)(&"1.5e-4".to_string()));
}

#[test]
fn test_number_string_special_cases() {
    let validator = umt_number_string(None);
    assert!(!(validator.validate)(&"NaN".to_string()));
    assert!(!(validator.validate)(&"Infinity".to_string()));
}
