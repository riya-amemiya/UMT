use std::collections::HashMap;
use umt_rust::object::{Value, umt_pick};

#[test]
fn test_should_select_a_single_key() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_pick(&obj, &["a"]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_select_multiple_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_pick(&obj, &["a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    let obj = Value::Object(map);

    let result = umt_pick(&obj, &["c"]);

    assert!(result.is_empty());
}
