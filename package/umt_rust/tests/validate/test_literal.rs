//! Tests for literal validation

use std::collections::HashMap;
use umt_rust::object::Value;
use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::umt_union;
use umt_rust::validate::umt_literal;

fn s(value: &str) -> Value {
    Value::String(value.to_string())
}

#[test]
fn test_literal_accepts_only_the_given_string() {
    let validator = umt_literal(s("admin"), None);
    let result = validator(&s("admin"));
    assert!(result.validate);
    assert_eq!(result.message, "");
    assert_eq!(result.type_value, s("admin"));
    assert!(!validator(&s("user")).validate);
}

#[test]
fn test_literal_accepts_only_the_given_number() {
    let validator = umt_literal(Value::Int(42), None);
    assert!(validator(&Value::Int(42)).validate);
    assert!(validator(&Value::Float(42.0)).validate);
    assert!(!validator(&Value::Int(43)).validate);
    assert!(!validator(&s("42")).validate);

    let float_validator = umt_literal(Value::Float(1.5), None);
    assert!(float_validator(&Value::Float(1.5)).validate);
    assert!(!float_validator(&Value::Int(1)).validate);
}

#[test]
fn test_literal_accepts_only_the_given_boolean() {
    let validator = umt_literal(Value::Bool(true), None);
    assert!(validator(&Value::Bool(true)).validate);
    assert!(!validator(&Value::Bool(false)).validate);
    assert!(!validator(&Value::Int(1)).validate);
}

#[test]
fn test_literal_accepts_null() {
    let validator = umt_literal(Value::Null, None);
    assert!(validator(&Value::Null).validate);
    assert!(!validator(&Value::Int(0)).validate);
    assert!(!validator(&s("")).validate);
}

#[test]
fn test_literal_uses_strict_equality_for_nan_and_signed_zero() {
    assert!(!umt_literal(Value::Float(f64::NAN), None)(&Value::Float(f64::NAN)).validate);
    assert!(umt_literal(Value::Int(0), None)(&Value::Float(-0.0)).validate);
    assert!(umt_literal(Value::Float(0.0), None)(&Value::Float(-0.0)).validate);
}

#[test]
fn test_literal_never_matches_arrays_or_objects() {
    let array = Value::Array(vec![Value::Int(1)]);
    assert!(!umt_literal(array.clone(), None)(&array).validate);
    let object = Value::Object(HashMap::new());
    assert!(!umt_literal(object.clone(), None)(&object).validate);
}

#[test]
fn test_literal_returns_the_custom_message_on_failure_only() {
    let validator = umt_literal(s("a"), Some("must be a".to_string()));
    assert_eq!(validator(&s("b")).message, "must be a");
    assert_eq!(validator(&s("a")).message, "");
}

#[test]
fn test_literal_defaults_the_message_to_empty_string() {
    assert_eq!(umt_literal(s("a"), None)(&s("b")).message, "");
}

#[test]
fn test_literal_composes_with_union() {
    let validator = umt_union(vec![
        Box::new(umt_literal(s("on"), Some("not on".to_string())))
            as Box<dyn Fn(&Value) -> ValidateCoreReturnType<Value>>,
        Box::new(umt_literal(s("off"), Some("not off".to_string()))),
    ]);
    assert!(validator(&s("on")).validate);
    assert!(validator(&s("off")).validate);
    assert_eq!(validator(&s("auto")).message, "not off");
}
