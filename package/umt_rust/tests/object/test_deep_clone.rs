use std::collections::HashMap;
use umt_rust::object::{Value, umt_deep_clone};

#[test]
fn test_clone_primitive_values() {
    assert_eq!(umt_deep_clone(&Value::Null), Value::Null);
    assert_eq!(umt_deep_clone(&Value::Bool(true)), Value::Bool(true));
    assert_eq!(umt_deep_clone(&Value::Int(42)), Value::Int(42));
    assert_eq!(umt_deep_clone(&Value::Float(3.14)), Value::Float(3.14));
    assert_eq!(
        umt_deep_clone(&Value::String("hello".to_string())),
        Value::String("hello".to_string())
    );
}

#[test]
fn test_clone_array() {
    let arr = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    let cloned = umt_deep_clone(&arr);
    assert_eq!(cloned, arr);
}

#[test]
fn test_clone_nested_object() {
    let mut inner = HashMap::new();
    inner.insert("c".to_string(), Value::Int(2));

    let mut original = HashMap::new();
    original.insert("a".to_string(), Value::Int(1));
    original.insert("b".to_string(), Value::Object(inner));

    let cloned = umt_deep_clone(&Value::Object(original.clone()));

    assert_eq!(cloned, Value::Object(original));
}

#[test]
fn test_clone_is_independent() {
    let mut inner = HashMap::new();
    inner.insert("x".to_string(), Value::Int(10));

    let mut original = HashMap::new();
    original.insert("nested".to_string(), Value::Object(inner));

    let original_value = Value::Object(original);
    let cloned = umt_deep_clone(&original_value);

    // Both should have the same content
    assert_eq!(original_value, cloned);
}

#[test]
fn test_filters_dangerous_keys() {
    let mut obj = HashMap::new();
    obj.insert("safe".to_string(), Value::Int(1));
    obj.insert("__proto__".to_string(), Value::Int(2));
    obj.insert("constructor".to_string(), Value::Int(3));
    obj.insert("prototype".to_string(), Value::Int(4));
    obj.insert("also_safe".to_string(), Value::Int(5));

    let cloned = umt_deep_clone(&Value::Object(obj));

    if let Value::Object(map) = &cloned {
        assert_eq!(map.get("safe"), Some(&Value::Int(1)));
        assert_eq!(map.get("also_safe"), Some(&Value::Int(5)));
        assert_eq!(map.get("__proto__"), None);
        assert_eq!(map.get("constructor"), None);
        assert_eq!(map.get("prototype"), None);
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_filters_dangerous_keys_in_nested_objects() {
    let mut nested = HashMap::new();
    nested.insert("valid".to_string(), Value::Int(1));
    nested.insert("__proto__".to_string(), Value::Int(2));

    let mut obj = HashMap::new();
    obj.insert("nested".to_string(), Value::Object(nested));

    let cloned = umt_deep_clone(&Value::Object(obj));

    if let Value::Object(map) = &cloned {
        if let Some(Value::Object(inner)) = map.get("nested") {
            assert_eq!(inner.get("valid"), Some(&Value::Int(1)));
            assert_eq!(inner.get("__proto__"), None);
        } else {
            panic!("Expected nested Object");
        }
    } else {
        panic!("Expected Object");
    }
}

#[test]
fn test_clone_empty_structures() {
    assert_eq!(umt_deep_clone(&Value::Array(vec![])), Value::Array(vec![]));
    assert_eq!(
        umt_deep_clone(&Value::Object(HashMap::new())),
        Value::Object(HashMap::new())
    );
}
