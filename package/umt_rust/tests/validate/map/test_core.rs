//! Tests for map core validation

use std::collections::HashMap;
use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::map::umt_validate_map;

fn string_validator(message: &'static str) -> impl Fn(&String) -> ValidateCoreReturnType<String> {
    move |v: &String| ValidateCoreReturnType {
        validate: true,
        message: message.to_string(),
        type_value: v.clone(),
    }
}

fn number_validator(
    message: &'static str,
) -> impl Fn(&i32) -> ValidateCoreReturnType<i32> {
    move |v: &i32| ValidateCoreReturnType {
        validate: true,
        message: message.to_string(),
        type_value: *v,
    }
}

#[test]
fn test_validate_map_with_no_validators() {
    let empty: HashMap<String, i32> = HashMap::new();
    let result = umt_validate_map(
        &empty,
        None::<fn(&String) -> ValidateCoreReturnType<String>>,
        None::<fn(&i32) -> ValidateCoreReturnType<i32>>,
        None,
    );
    assert!(result.validate);

    let mut filled: HashMap<String, i32> = HashMap::new();
    filled.insert("a".to_string(), 1);
    let result = umt_validate_map(
        &filled,
        None::<fn(&String) -> ValidateCoreReturnType<String>>,
        None::<fn(&i32) -> ValidateCoreReturnType<i32>>,
        None,
    );
    assert!(result.validate);
}

#[test]
fn test_validate_map_validates_keys_and_values() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("a".to_string(), 1);
    m.insert("b".to_string(), 2);
    let result = umt_validate_map(
        &m,
        Some(string_validator("")),
        Some(number_validator("")),
        None,
    );
    assert!(result.validate);
}

#[test]
fn test_validate_map_fails_on_invalid_key() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("a".to_string(), 1);
    let key_fail = |v: &String| ValidateCoreReturnType {
        validate: false,
        message: "key must be valid".to_string(),
        type_value: v.clone(),
    };
    let result = umt_validate_map(
        &m,
        Some(key_fail),
        None::<fn(&i32) -> ValidateCoreReturnType<i32>>,
        None,
    );
    assert!(!result.validate);
}

#[test]
fn test_validate_map_fails_on_invalid_value() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("a".to_string(), 1);
    let value_fail = |v: &i32| ValidateCoreReturnType {
        validate: false,
        message: "value must be valid".to_string(),
        type_value: *v,
    };
    let result = umt_validate_map(
        &m,
        None::<fn(&String) -> ValidateCoreReturnType<String>>,
        Some(value_fail),
        None,
    );
    assert!(!result.validate);
}

#[test]
fn test_validate_map_propagates_failing_entry_message() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("a".to_string(), 1);
    let value_fail = |v: &i32| ValidateCoreReturnType {
        validate: false,
        message: "value must be number".to_string(),
        type_value: *v,
    };
    let result = umt_validate_map(
        &m,
        Some(string_validator("key must be string")),
        Some(value_fail),
        None,
    );
    assert!(!result.validate);
    assert_eq!(result.message, "value must be number");
}

#[test]
fn test_validate_map_returns_configured_message_on_success() {
    let m: HashMap<String, i32> = HashMap::new();
    let result = umt_validate_map(
        &m,
        None::<fn(&String) -> ValidateCoreReturnType<String>>,
        None::<fn(&i32) -> ValidateCoreReturnType<i32>>,
        Some("must be a Map"),
    );
    assert!(result.validate);
    assert_eq!(result.message, "must be a Map");
}

#[test]
fn test_validate_map_preserves_original_map() {
    let mut m: HashMap<String, i32> = HashMap::new();
    m.insert("a".to_string(), 1);
    let result = umt_validate_map(
        &m,
        None::<fn(&String) -> ValidateCoreReturnType<String>>,
        None::<fn(&i32) -> ValidateCoreReturnType<i32>>,
        None,
    );
    assert_eq!(result.type_value, m);
}
