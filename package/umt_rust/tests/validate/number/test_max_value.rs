//! Tests for max value validation

use umt_rust::validate::number::umt_max_value;

#[test]
fn test_max_value_returns_true_for_numbers_less_than_or_equal_to_max() {
    let validator = umt_max_value(10.0, None);
    assert!((validator.validate)(&5.0));
    assert!((validator.validate)(&10.0));
}

#[test]
fn test_max_value_returns_false_for_numbers_greater_than_max() {
    let validator = umt_max_value(10.0, None);
    assert!(!(validator.validate)(&11.0));
}

#[test]
fn test_max_value_returns_custom_message() {
    let custom_message = "Value must be less than or equal to 10";
    let validator = umt_max_value(10.0, Some(custom_message.to_string()));
    assert!(!(validator.validate)(&11.0));
    assert_eq!(validator.message, Some(custom_message.to_string()));
}

#[test]
fn test_max_value_with_negative_numbers() {
    let validator = umt_max_value(-5.0, None);
    assert!((validator.validate)(&-10.0));
    assert!((validator.validate)(&-5.0));
    assert!(!(validator.validate)(&-4.0));
    assert!(!(validator.validate)(&0.0));
}

#[test]
fn test_max_value_with_zero() {
    let validator = umt_max_value(0.0, None);
    assert!((validator.validate)(&0.0));
    assert!((validator.validate)(&-1.0));
    assert!(!(validator.validate)(&1.0));
}

#[test]
fn test_max_value_with_decimal_numbers() {
    let validator = umt_max_value(5.5, None);
    assert!((validator.validate)(&5.4));
    assert!((validator.validate)(&5.5));
    assert!(!(validator.validate)(&5.6));
}
