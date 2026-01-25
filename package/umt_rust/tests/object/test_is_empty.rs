use std::collections::HashMap;
use umt_rust::object::Value;

/// Helper function to check if a HashMap is empty (mirrors the TypeScript isEmpty)
fn is_empty(obj: &HashMap<String, Value>) -> bool {
    obj.is_empty()
}

#[test]
fn test_should_return_true_for_empty_object() {
    let obj: HashMap<String, Value> = HashMap::new();
    assert!(is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_properties() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_multiple_properties() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_null_values() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Null);
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_false_values() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Bool(false));
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_zero_values() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(0));
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_empty_string_values() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::String("".to_string()));
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_empty_array_values() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Array(vec![]));
    assert!(!is_empty(&obj));
}

#[test]
fn test_should_return_false_for_object_with_nested_empty_object_values() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(HashMap::new()));
    assert!(!is_empty(&obj));
}
