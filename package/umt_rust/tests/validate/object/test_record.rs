//! Tests for record validation

use std::collections::HashMap;
use umt_rust::object::Value;
use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::umt_record;
use umt_rust::validate::string::{umt_min_length, umt_one_of, umt_string_validator};

fn is_number(message: &'static str) -> impl Fn(&Value) -> ValidateCoreReturnType<Value> {
    move |value: &Value| ValidateCoreReturnType {
        validate: matches!(value, Value::Int(_) | Value::Float(_)),
        message: message.to_string(),
        type_value: value.clone(),
    }
}

fn object(entries: &[(&str, Value)]) -> Value {
    Value::Object(
        entries
            .iter()
            .map(|(key, value)| (key.to_string(), value.clone()))
            .collect(),
    )
}

#[test]
fn test_record_accepts_an_empty_object() {
    let validator = umt_record(umt_string_validator(vec![], None), is_number(""), None);
    let value = Value::Object(HashMap::new());
    let result = validator(&value);
    assert!(result.validate);
    assert_eq!(result.message, "");
    assert_eq!(result.type_value, value);
}

#[test]
fn test_record_validates_every_key_and_value() {
    let validator = umt_record(umt_string_validator(vec![], None), is_number(""), None);
    let value = object(&[("a", Value::Int(1)), ("b", Value::Float(2.5))]);
    assert!(validator(&value).validate);
}

#[test]
fn test_record_propagates_the_failing_value_message() {
    let validator = umt_record(
        umt_string_validator(vec![], None),
        is_number("value must be a number"),
        None,
    );
    let value = object(&[("a", Value::Int(1)), ("b", Value::String("x".to_string()))]);
    let result = validator(&value);
    assert!(!result.validate);
    assert_eq!(result.message, "value must be a number");
    assert_eq!(result.type_value, value);
}

#[test]
fn test_record_propagates_the_failing_key_message() {
    let validator = umt_record(
        umt_string_validator(
            vec![umt_min_length(2, Some("key too short".to_string()))],
            None,
        ),
        is_number("value must be a number"),
        None,
    );
    let result = validator(&object(&[("ab", Value::Int(1)), ("c", Value::Int(2))]));
    assert!(!result.validate);
    assert_eq!(result.message, "key too short");
}

#[test]
fn test_record_checks_the_key_before_its_value() {
    let validator = umt_record(
        umt_string_validator(
            vec![umt_min_length(2, Some("key too short".to_string()))],
            None,
        ),
        is_number("value must be a number"),
        None,
    );
    let value = object(&[("c", Value::String("x".to_string()))]);
    assert_eq!(validator(&value).message, "key too short");
}

#[test]
fn test_record_visits_keys_in_ascending_order() {
    let validator = umt_record(
        |key: &str| ValidateCoreReturnType {
            validate: false,
            message: key.to_string(),
            type_value: key.to_string(),
        },
        is_number(""),
        None,
    );
    let value = object(&[
        ("c", Value::Int(1)),
        ("a", Value::Int(1)),
        ("b", Value::Int(1)),
    ]);
    assert_eq!(validator(&value).message, "a");
}

#[test]
fn test_record_restricts_keys_with_one_of() {
    let validator = umt_record(
        umt_one_of(&["en", "ja"], Some("unknown locale".to_string())),
        umt_string_validator_value(),
        None,
    );
    let valid = object(&[("en", Value::String("Hello".to_string()))]);
    assert!(validator(&valid).validate);
    let invalid = object(&[
        ("en", Value::String("Hello".to_string())),
        ("fr", Value::String("Bonjour".to_string())),
    ]);
    assert_eq!(validator(&invalid).message, "unknown locale");
}

fn umt_string_validator_value() -> impl Fn(&Value) -> ValidateCoreReturnType<Value> {
    |value: &Value| ValidateCoreReturnType {
        validate: matches!(value, Value::String(_)),
        message: "not a string".to_string(),
        type_value: value.clone(),
    }
}

#[test]
fn test_record_rejects_non_object_values() {
    let validator = umt_record(
        umt_string_validator(vec![], None),
        is_number(""),
        Some("must be a record".to_string()),
    );
    for value in [
        Value::Null,
        Value::Array(vec![Value::Int(1)]),
        Value::String("a".to_string()),
    ] {
        let result = validator(&value);
        assert!(!result.validate);
        assert_eq!(result.message, "must be a record");
        assert_eq!(result.type_value, value);
    }
}

#[test]
fn test_record_defaults_the_message_to_empty_string() {
    let validator = umt_record(umt_string_validator(vec![], None), is_number(""), None);
    assert_eq!(validator(&Value::Null).message, "");
}
