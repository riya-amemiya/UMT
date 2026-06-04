//! Tests for function core validation

use umt_rust::validate::core::ValidateReturnType;
use umt_rust::validate::function::{umt_func_implement, umt_validate_func};

#[test]
fn test_validate_func_accepts_callable_values() {
    let result = umt_validate_func(|x: i32| x + 1, None);
    assert!(result.validate);
    assert_eq!((result.type_value)(1), 2);
}

#[test]
fn test_validate_func_returns_configured_message() {
    let result = umt_validate_func(|x: i32| x, Some("must be a function"));
    assert!(result.validate);
    assert_eq!(result.message, "must be a function");
}

#[test]
fn test_implement_validates_input() {
    let positive = ValidateReturnType::new("number", None, |value: &f64| *value > 0.0);
    let wrapped = umt_func_implement(|n: f64| n * 2.0, Some(positive), None);
    assert_eq!(wrapped(2.0), Ok(4.0));
    assert!(wrapped(-1.0).is_err());
}

#[test]
fn test_implement_validates_output() {
    let positive_output = ValidateReturnType::new("number", None, |value: &f64| *value > 0.0);
    let wrapped = umt_func_implement(|n: f64| n - 5.0, None, Some(positive_output));
    assert_eq!(wrapped(10.0), Ok(5.0));
    assert!(wrapped(1.0).is_err());
}

#[test]
fn test_implement_input_default_message() {
    let always_fail = ValidateReturnType::new("number", None, |_: &f64| false);
    let wrapped = umt_func_implement(|n: f64| n, Some(always_fail), None);
    assert_eq!(
        wrapped(1.0),
        Err("function input at index 0 failed validation".to_string())
    );
}

#[test]
fn test_implement_output_default_message() {
    let always_fail = ValidateReturnType::new("number", None, |_: &f64| false);
    let wrapped = umt_func_implement(|n: f64| n, None, Some(always_fail));
    assert_eq!(
        wrapped(1.0),
        Err("function output failed validation".to_string())
    );
}

#[test]
fn test_implement_uses_custom_validator_message() {
    let always_fail = ValidateReturnType::new(
        "number",
        Some("custom input message".to_string()),
        |_: &f64| false,
    );
    let wrapped = umt_func_implement(|n: f64| n, Some(always_fail), None);
    assert_eq!(wrapped(1.0), Err("custom input message".to_string()));
}

#[test]
fn test_implement_returns_value_untouched_without_validators() {
    let wrapped = umt_func_implement(|value: i32| value * 2, None, None);
    assert_eq!(wrapped(3), Ok(6));
}

#[test]
fn test_implement_does_not_change_call_results() {
    let positive = ValidateReturnType::new("number", None, |value: &f64| *value > 0.0);
    let positive_output = ValidateReturnType::new("number", None, |value: &f64| *value > 0.0);
    let wrapped = umt_func_implement(|n: f64| n * 2.0, Some(positive), Some(positive_output));
    assert_eq!(wrapped(2.0), Ok(4.0));
    assert_eq!(wrapped(3.0), Ok(6.0));
}
