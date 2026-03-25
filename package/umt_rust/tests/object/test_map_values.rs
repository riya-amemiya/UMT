use std::collections::HashMap;

use umt_rust::object::{Value, umt_map_values};

#[test]
fn test_transform_values_using_provided_function() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));

    let result = umt_map_values(&map, |value, _| match value {
        Value::Int(v) => Value::Int(v * 2),
        other => other.clone(),
    });

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Int(2));
    expected.insert("b".to_string(), Value::Int(4));
    expected.insert("c".to_string(), Value::Int(6));

    assert_eq!(result, expected);
}

#[test]
fn test_pass_value_and_key_to_transformer() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), Value::Int(10));
    map.insert("y".to_string(), Value::Int(20));

    let result = umt_map_values(&map, |value, key| match value {
        Value::Int(v) => Value::String(format!("{}={}", key, v)),
        other => other.clone(),
    });

    let mut expected = HashMap::new();
    expected.insert("x".to_string(), Value::String("x=10".to_string()));
    expected.insert("y".to_string(), Value::String("y=20".to_string()));

    assert_eq!(result, expected);
}

#[test]
fn test_handle_empty_object() {
    let map = HashMap::new();

    let result = umt_map_values(&map, |value, _| value.clone());

    let expected: HashMap<String, Value> = HashMap::new();

    assert_eq!(result, expected);
}

#[test]
fn test_not_modify_original_object() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let original = map.clone();

    let _result = umt_map_values(&map, |value, _| match value {
        Value::Int(v) => Value::Int(v * 2),
        other => other.clone(),
    });

    assert_eq!(map, original);
}

#[test]
fn test_handle_different_return_types() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));

    let result = umt_map_values(&map, |value, _| match value {
        Value::Int(v) => Value::Bool(*v > 1),
        _ => Value::Bool(false),
    });

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Bool(false));
    expected.insert("b".to_string(), Value::Bool(true));
    expected.insert("c".to_string(), Value::Bool(true));

    assert_eq!(result, expected);
}
