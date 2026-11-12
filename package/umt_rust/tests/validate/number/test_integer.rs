//! Tests for integer number validation

use umt_rust::validate::number::{umt_integer, umt_number_validator};

#[test]
fn test_integer_returns_true_for_integers() {
    let validator = umt_integer(None);
    for value in [0.0, 1.0, -1.0, 9_007_199_254_740_991.0, 1e21, -0.0] {
        assert!((validator.validate)(&value), "{value}");
    }
}

#[test]
fn test_integer_returns_false_for_non_integers() {
    let validator = umt_integer(None);
    for value in [1.5, -0.1, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(!(validator.validate)(&value), "{value}");
    }
}

#[test]
fn test_integer_carries_the_message() {
    assert_eq!(
        umt_integer(Some("must be an integer".to_string())).message,
        Some("must be an integer".to_string())
    );
    assert_eq!(umt_integer(None).message, None);
    assert_eq!(umt_integer(None).type_name, "number");
}

#[test]
fn test_integer_composes_with_number_validator() {
    let validator = umt_number_validator(
        vec![umt_integer(Some("must be an integer".to_string()))],
        None,
    );
    assert!(validator(3.0).validate);
    let result = validator(3.5);
    assert!(!result.validate);
    assert_eq!(result.message, "must be an integer");
}
