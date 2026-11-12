//! Tests for tuple validation

use std::collections::HashMap;
use umt_rust::object::Value;
use umt_rust::validate::array::umt_tuple;
use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::umt_literal;

type BoxedValidator = Box<dyn Fn(&Value) -> ValidateCoreReturnType<Value>>;

fn is_string(message: &str) -> BoxedValidator {
    let message = message.to_string();
    Box::new(move |value: &Value| ValidateCoreReturnType {
        validate: matches!(value, Value::String(_)),
        message: message.clone(),
        type_value: value.clone(),
    })
}

fn is_number(message: &str) -> BoxedValidator {
    let message = message.to_string();
    Box::new(move |value: &Value| ValidateCoreReturnType {
        validate: matches!(value, Value::Int(_) | Value::Float(_)),
        message: message.clone(),
        type_value: value.clone(),
    })
}

fn s(value: &str) -> Value {
    Value::String(value.to_string())
}

#[test]
fn test_tuple_validates_each_element_with_the_validator_at_the_same_index() {
    let validator = umt_tuple(vec![is_string("a"), is_number("b")], None);
    let value = Value::Array(vec![s("a"), Value::Int(1)]);
    let result = validator(&value);
    assert!(result.validate);
    assert_eq!(result.message, "");
    assert_eq!(result.type_value, value);
}

#[test]
fn test_tuple_propagates_the_message_of_the_first_failing_element() {
    let validator = umt_tuple(
        vec![
            is_string("first must be a string"),
            is_number("second must be a number"),
        ],
        None,
    );
    let second = validator(&Value::Array(vec![s("a"), s("b")]));
    assert!(!second.validate);
    assert_eq!(second.message, "second must be a number");
    let first = validator(&Value::Array(vec![Value::Int(1), s("b")]));
    assert_eq!(first.message, "first must be a string");
}

#[test]
fn test_tuple_rejects_arrays_whose_length_differs() {
    let validator = umt_tuple(
        vec![is_string(""), is_number("")],
        Some("must be [string, number]".to_string()),
    );
    for value in [
        Value::Array(vec![s("a")]),
        Value::Array(vec![s("a"), Value::Int(1), Value::Int(2)]),
    ] {
        let result = validator(&value);
        assert!(!result.validate);
        assert_eq!(result.message, "must be [string, number]");
        assert_eq!(result.type_value, value);
    }
}

#[test]
fn test_tuple_rejects_non_array_values() {
    let validator = umt_tuple(vec![is_string("")], Some("not a tuple".to_string()));
    for value in [Value::Null, s("a"), Value::Object(HashMap::new())] {
        let result = validator(&value);
        assert!(!result.validate);
        assert_eq!(result.message, "not a tuple");
    }
}

#[test]
fn test_tuple_defaults_the_message_to_empty_string() {
    let validator = umt_tuple(vec![is_string("")], None);
    assert_eq!(validator(&Value::Null).message, "");
}

#[test]
fn test_tuple_accepts_only_the_empty_array_for_an_empty_tuple() {
    let validator = umt_tuple::<Value>(vec![], None);
    assert!(validator(&Value::Array(vec![])).validate);
    assert!(!validator(&Value::Array(vec![Value::Int(1)])).validate);
}

#[test]
fn test_tuple_composes_with_literal() {
    let validator = umt_tuple(
        vec![
            Box::new(umt_literal(s("x"), Some("must be x".to_string()))) as BoxedValidator,
            is_string(""),
        ],
        None,
    );
    assert!(validator(&Value::Array(vec![s("x"), s("y")])).validate);
    assert_eq!(
        validator(&Value::Array(vec![s("z"), s("y")])).message,
        "must be x"
    );
}
