//! Tests for even number validation

use umt_rust::validate::number::umt_even;

#[test]
fn test_even_returns_true_for_even_numbers() {
    let validator = umt_even(None);
    assert!((validator.validate)(&2.0));
    assert!((validator.validate)(&4.0));
    assert!((validator.validate)(&6.0));
    assert!((validator.validate)(&8.0));
    assert!((validator.validate)(&10.0));
    assert!((validator.validate)(&100.0));
    assert!((validator.validate)(&1000.0));
}

#[test]
fn test_even_returns_false_for_odd_numbers() {
    let validator = umt_even(None);
    assert!(!(validator.validate)(&1.0));
    assert!(!(validator.validate)(&3.0));
    assert!(!(validator.validate)(&5.0));
    assert!(!(validator.validate)(&7.0));
    assert!(!(validator.validate)(&9.0));
    assert!(!(validator.validate)(&99.0));
    assert!(!(validator.validate)(&999.0));
}

#[test]
fn test_even_returns_custom_message() {
    let custom_message = "This is a custom message";
    let validator = umt_even(Some(custom_message.to_string()));
    assert!(!(validator.validate)(&1.0));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_even_handles_zero() {
    let validator = umt_even(None);
    assert!((validator.validate)(&0.0));
}

#[test]
fn test_even_handles_negative_numbers() {
    let validator = umt_even(None);
    assert!((validator.validate)(&-2.0));
    assert!((validator.validate)(&-4.0));
    assert!((validator.validate)(&-6.0));
    assert!(!(validator.validate)(&-1.0));
    assert!(!(validator.validate)(&-3.0));
    assert!(!(validator.validate)(&-5.0));
}

#[test]
fn test_even_handles_decimal_numbers() {
    let validator = umt_even(None);
    assert!(!(validator.validate)(&2.5));
    assert!(!(validator.validate)(&4.7));
    assert!(!(validator.validate)(&1.3));
    assert!(!(validator.validate)(&3.9));
}

#[test]
fn test_even_handles_large_numbers() {
    let validator = umt_even(None);
    assert!((validator.validate)(&1_000_000.0));
    assert!(!(validator.validate)(&999_999.0));

    // Due to floating point precision, we test with a slightly smaller value
    assert!((validator.validate)(&1_000_000_000_000.0));
}
