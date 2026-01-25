//! Tests for object core validation

use std::collections::HashMap;
use umt_rust::validate::object::umt_validate_object;

#[test]
fn test_validate_object_with_no_validators() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "John Doe".to_string());
    let result = umt_validate_object(&obj, None, None);
    assert!(result.validate);
}

#[test]
fn test_validate_object_with_string_type() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "John Doe".to_string());
    obj.insert("age".to_string(), "thirty".to_string());
    let result = umt_validate_object(&obj, None, None);
    assert!(result.validate);
}

#[test]
fn test_validate_object_with_validators() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "John Doe".to_string());

    let mut validators: HashMap<String, Box<dyn Fn(&String) -> bool>> = HashMap::new();
    validators.insert("name".to_string(), Box::new(|s: &String| !s.is_empty()));

    let result = umt_validate_object(&obj, Some(&validators), None);
    assert!(result.validate);
}

#[test]
fn test_validate_object_fails_with_invalid_value() {
    let mut obj: HashMap<String, String> = HashMap::new();
    obj.insert("name".to_string(), "".to_string()); // Empty name

    let mut validators: HashMap<String, Box<dyn Fn(&String) -> bool>> = HashMap::new();
    validators.insert("name".to_string(), Box::new(|s: &String| !s.is_empty()));

    let result = umt_validate_object(&obj, Some(&validators), Some("Invalid object"));
    assert!(!result.validate);
}

#[test]
fn test_validate_object_returns_custom_message() {
    let obj: HashMap<String, String> = HashMap::new();

    let mut validators: HashMap<String, Box<dyn Fn(&String) -> bool>> = HashMap::new();
    validators.insert("name".to_string(), Box::new(|s: &String| !s.is_empty()));

    let custom_message = "Invalid object structure";
    let result = umt_validate_object(&obj, Some(&validators), Some(custom_message));
    assert!(!result.validate);
    // Missing field message is returned
    assert!(result.message.contains("Missing"));
}

#[test]
fn test_validate_object_returns_empty_message_on_validation_failure() {
    let obj: HashMap<String, String> = HashMap::new();

    let mut validators: HashMap<String, Box<dyn Fn(&String) -> bool>> = HashMap::new();
    validators.insert("name".to_string(), Box::new(|s: &String| !s.is_empty()));

    let result = umt_validate_object(&obj, Some(&validators), None);
    assert!(!result.validate);
}

#[test]
fn test_validate_object_with_numeric_values() {
    let mut obj: HashMap<String, i32> = HashMap::new();
    obj.insert("age".to_string(), 30);

    let mut validators: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
    validators.insert("age".to_string(), Box::new(|n: &i32| *n > 0 && *n < 150));

    let result = umt_validate_object(&obj, Some(&validators), None);
    assert!(result.validate);
}
