use std::collections::HashMap;
use umt_rust::object::{Value, umt_omit};

#[test]
fn test_should_omit_specified_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_omit(&obj, &["b"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
    assert!(result.get("b").is_none());
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    let obj = Value::Object(map);

    let result = umt_omit(&obj, &["c"]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}
