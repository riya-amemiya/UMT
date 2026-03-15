use std::collections::HashMap;

use umt_rust::object::{umt_map_keys, Value};

#[test]
fn test_transform_keys_using_provided_function() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));

    let result = umt_map_keys(&map, |_, key| key.to_uppercase());

    let mut expected = HashMap::new();
    expected.insert("A".to_string(), Value::Int(1));
    expected.insert("B".to_string(), Value::Int(2));
    expected.insert("C".to_string(), Value::Int(3));

    assert_eq!(result, expected);
}

#[test]
fn test_pass_value_and_key_to_transformer() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), Value::Int(10));
    map.insert("y".to_string(), Value::Int(20));

    let result = umt_map_keys(&map, |value, key| match value {
        Value::Int(v) => format!("{}_{}", key, v),
        _ => key.to_string(),
    });

    let mut expected = HashMap::new();
    expected.insert("x_10".to_string(), Value::Int(10));
    expected.insert("y_20".to_string(), Value::Int(20));

    assert_eq!(result, expected);
}

#[test]
fn test_handle_empty_object() {
    let map = HashMap::new();

    let result = umt_map_keys(&map, |_, key| key.to_string());

    let expected: HashMap<String, Value> = HashMap::new();

    assert_eq!(result, expected);
}

#[test]
fn test_handle_keys_that_map_to_same_value() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_map_keys(&map, |_, _| "same".to_string());

    // Since HashMap iteration order is not guaranteed, and both keys map to "same",
    // the value associated with "same" could be either 1 or 2. We check that it has exactly one key "same".
    assert_eq!(result.len(), 1);
    assert!(result.contains_key("same"));
    let val = result.get("same").unwrap();
    assert!(val == &Value::Int(1) || val == &Value::Int(2));
}

#[test]
fn test_not_modify_original_object() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let original = map.clone();

    let _result = umt_map_keys(&map, |_, key| key.to_uppercase());

    assert_eq!(map, original);
}
