//! Tests for number core validation

use umt_rust::validate::number::{umt_max_value, umt_min_value, umt_validate_number};

#[test]
fn test_validate_number_with_no_additional_options() {
    let result = umt_validate_number(5.0, &[], None);
    assert!(result.validate);
}

#[test]
fn test_validate_number_with_max_value_option() {
    let max_validator = umt_max_value(10.0, None);
    let result = umt_validate_number(5.0, &[max_validator.clone()], None);
    assert!(result.validate);

    let result_invalid = umt_validate_number(11.0, &[max_validator], None);
    assert!(!result_invalid.validate);
}

#[test]
fn test_validate_number_with_min_value_option() {
    let min_validator = umt_min_value(3.0, None);
    let result = umt_validate_number(5.0, &[min_validator.clone()], None);
    assert!(result.validate);

    let result_invalid = umt_validate_number(2.0, &[min_validator], None);
    assert!(!result_invalid.validate);
}

#[test]
fn test_validate_number_with_both_min_and_max_value_options() {
    let min_validator = umt_min_value(3.0, None);
    let max_validator = umt_max_value(10.0, None);
    let validators = vec![min_validator.clone(), max_validator.clone()];

    let result = umt_validate_number(5.0, &validators, None);
    assert!(result.validate);

    let min_validator2 = umt_min_value(3.0, None);
    let max_validator2 = umt_max_value(10.0, None);
    let result_below = umt_validate_number(2.0, &[min_validator2, max_validator2], None);
    assert!(!result_below.validate);

    let min_validator3 = umt_min_value(3.0, None);
    let max_validator3 = umt_max_value(10.0, None);
    let result_above = umt_validate_number(11.0, &[min_validator3, max_validator3], None);
    assert!(!result_above.validate);
}

#[test]
fn test_validate_number_returns_custom_message_on_failure() {
    let custom_message = "Value must be between 3 and 10";
    let min_validator = umt_min_value(3.0, Some(custom_message.to_string()));
    let max_validator = umt_max_value(10.0, None);

    let result = umt_validate_number(2.0, &[min_validator, max_validator], None);
    assert!(!result.validate);
    assert_eq!(result.message, custom_message);
}

#[test]
fn test_validate_number_rejects_nan() {
    let result = umt_validate_number(f64::NAN, &[], None);
    assert!(!result.validate);
}

#[test]
fn test_validate_number_rejects_infinity() {
    let result = umt_validate_number(f64::INFINITY, &[], None);
    assert!(!result.validate);

    let result_neg = umt_validate_number(f64::NEG_INFINITY, &[], None);
    assert!(!result_neg.validate);
}
