//! Integration tests for JSON parsing and validation
//!
//! Tests the interaction between parse_json and validate functions to ensure:
//! - JSON strings are correctly parsed into objects
//! - Parsed objects match the expected schema (using validators)
//! - Type validation works for various data types (number, boolean)
//! - Nested object validation works correctly

use serde_json::Value;
use std::collections::HashMap;
use umt_rust::tool::umt_parse_json;
use umt_rust::validate::object::umt_validate_object;

#[test]
fn should_parse_json_string_with_number_values() {
    let json_string = r#"{"key": 123}"#;
    let result: HashMap<String, Value> =
        umt_parse_json(json_string).expect("JSON parsing should succeed for valid input");

    let mut validators: HashMap<String, Box<dyn Fn(&Value) -> bool>> = HashMap::new();
    validators.insert(
        "key".to_string(),
        Box::new(|v| v.is_number()),
    );

    let validation = umt_validate_object(&result, Some(&validators), None);
    assert!(validation.validate);
    assert_eq!(
        result.get("key").expect("Key 'key' should exist").as_i64(),
        Some(123)
    );
}

#[test]
fn should_parse_json_string_with_boolean_values() {
    let json_string = r#"{"key": true}"#;
    let result: HashMap<String, Value> =
        umt_parse_json(json_string).expect("JSON parsing should succeed for valid input");

    let mut validators: HashMap<String, Box<dyn Fn(&Value) -> bool>> = HashMap::new();
    validators.insert(
        "key".to_string(),
        Box::new(|v| v.is_boolean()),
    );

    let validation = umt_validate_object(&result, Some(&validators), None);
    assert!(validation.validate);
    assert_eq!(
        result.get("key").expect("Key 'key' should exist").as_bool(),
        Some(true)
    );
}

#[test]
fn should_validate_nested_objects() {
    let json_string = r#"{"user": {"id": 1, "active": true}}"#;
    let result: HashMap<String, Value> =
        umt_parse_json(json_string).expect("JSON parsing should succeed for valid input");

    let mut validators: HashMap<String, Box<dyn Fn(&Value) -> bool>> = HashMap::new();
    validators.insert(
        "user".to_string(),
        Box::new(|v| {
            if let Some(obj) = v.as_object() {
                // Convert serde Map to HashMap for umt_validate_object
                let mut hm = HashMap::new();
                for (k, v) in obj {
                    hm.insert(k.clone(), v.clone());
                }

                let mut inner_validators: HashMap<String, Box<dyn Fn(&Value) -> bool>> =
                    HashMap::new();
                inner_validators.insert("id".to_string(), Box::new(|v| v.is_number()));
                inner_validators.insert("active".to_string(), Box::new(|v| v.is_boolean()));

                umt_validate_object(&hm, Some(&inner_validators), None).validate
            } else {
                false
            }
        }),
    );

    let validation = umt_validate_object(&result, Some(&validators), None);
    assert!(validation.validate);

    let user = result
        .get("user")
        .expect("Key 'user' should exist")
        .as_object()
        .expect("Value should be an object");
    assert_eq!(
        user.get("id").expect("Key 'id' should exist").as_i64(),
        Some(1)
    );
    assert_eq!(
        user.get("active")
            .expect("Key 'active' should exist")
            .as_bool(),
        Some(true)
    );
}

#[test]
fn should_handle_invalid_json_strings() {
    let invalid_json = "{key: invalid}";
    let result: Result<HashMap<String, Value>, _> = umt_parse_json(invalid_json);
    assert!(result.is_err());
}

#[test]
fn should_fail_validation_for_mismatched_types() {
    let json_string = r#"{"key": "123"}"#; // string instead of number
    let result: HashMap<String, Value> =
        umt_parse_json(json_string).expect("JSON parsing should succeed for valid input");

    let mut validators: HashMap<String, Box<dyn Fn(&Value) -> bool>> = HashMap::new();
    validators.insert(
        "key".to_string(),
        Box::new(|v| v.is_number()),
    );

    let validation = umt_validate_object(&result, Some(&validators), None);
    assert!(!validation.validate);
}

#[test]
fn should_fail_validation_for_missing_required_fields() {
    let json_string = r#"{"id": 1}"#; // missing 'active' field
    let result: HashMap<String, Value> =
        umt_parse_json(json_string).expect("JSON parsing should succeed for valid input");

    let mut validators: HashMap<String, Box<dyn Fn(&Value) -> bool>> = HashMap::new();
    validators.insert("id".to_string(), Box::new(|v| v.is_number()));
    validators.insert("active".to_string(), Box::new(|v| v.is_boolean()));

    let validation = umt_validate_object(&result, Some(&validators), None);
    assert!(!validation.validate);
    assert!(validation.message.contains("Missing required field"));
}
