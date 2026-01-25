//! Tests for boolean core validation

use umt_rust::validate::boolean::{umt_boolean_validator, umt_validate_boolean};

#[test]
fn test_validate_boolean_returns_true_for_boolean_values() {
    let result_true = umt_validate_boolean(true, None);
    assert!(result_true.validate);
    assert!(result_true.type_value);

    let result_false = umt_validate_boolean(false, None);
    assert!(result_false.validate);
    assert!(!result_false.type_value);
}

#[test]
fn test_validate_boolean_with_custom_message() {
    let custom_message = "This is not a boolean";
    let result = umt_validate_boolean(true, Some(custom_message));
    assert!(result.validate);
    assert_eq!(result.message, custom_message);
}

#[test]
fn test_boolean_validator_function() {
    let validator = umt_boolean_validator(None);
    let result = validator(true);
    assert!(result.validate);
    assert!(result.type_value);

    let result_false = validator(false);
    assert!(result_false.validate);
    assert!(!result_false.type_value);
}

#[test]
fn test_boolean_validator_with_message() {
    let custom_message = Some("Custom message".to_string());
    let validator = umt_boolean_validator(custom_message);
    let result = validator(true);
    assert!(result.validate);
    assert_eq!(result.message, "Custom message");
}

// Note: In Rust, the function is statically typed to accept bool,
// so we cannot test with non-boolean values as we would in TypeScript.
// The type system prevents invalid inputs at compile time.
