//! Tests for double number validation

use umt_rust::validate::number::umt_double;

#[test]
fn test_double_returns_true_for_double_numbers() {
    let validator = umt_double(None);
    assert!((validator.validate)(&1.5));
    assert!((validator.validate)(&2.22));
    assert!((validator.validate)(&0.1));
}

#[test]
fn test_double_returns_false_for_non_double_numbers() {
    let validator = umt_double(None);
    assert!(!(validator.validate)(&1.0));
    assert!(!(validator.validate)(&100.0));
    assert!(!(validator.validate)(&0.0));
}

#[test]
fn test_double_returns_custom_message() {
    let custom_message = "This is not a double number";
    let validator = umt_double(Some(custom_message.to_string()));
    assert!(!(validator.validate)(&1.0));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_double_with_negative_values() {
    let validator = umt_double(None);
    assert!((validator.validate)(&-1.5));
    assert!((validator.validate)(&-2.22));
    assert!(!(validator.validate)(&-1.0));
}

#[test]
fn test_double_with_very_small_fractions() {
    let validator = umt_double(None);
    assert!((validator.validate)(&0.001));
    assert!((validator.validate)(&0.0001));
}

use umt_rust::validate::number::*;

#[test]
fn test_custom_message() {
    let custom_message = "This is not a double number";
    let validator = umt_double(Some(custom_message.to_string()));
    assert!(!(validator.validate)(&1.0));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_returns_false_for_non_double_numbers() {
    let validator = umt_double(None);
    assert!(!(validator.validate)(&1.0));
    assert!(!(validator.validate)(&100.0));
    assert!(!(validator.validate)(&0.0));
}

#[test]
fn test_returns_true_for_double_numbers() {
    let validator = umt_double(None);
    assert!((validator.validate)(&1.5));
    assert!((validator.validate)(&2.22));
    assert!((validator.validate)(&0.1));
}
