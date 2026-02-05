use std::collections::HashMap;
use umt_rust::object::{Value, umt_merge};

#[test]
fn test_should_merge_objects() {
    let mut map1 = HashMap::new();
    map1.insert("a".to_string(), Value::Int(1));
    let obj1 = Value::Object(map1);

    let mut map2 = HashMap::new();
    map2.insert("b".to_string(), Value::Int(2));
    let obj2 = Value::Object(map2);

    let result = umt_merge(&obj1, &[obj2]);

    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_should_override_properties() {
    let mut map1 = HashMap::new();
    map1.insert("a".to_string(), Value::Int(1));
    let obj1 = Value::Object(map1);

    let mut map2 = HashMap::new();
    map2.insert("a".to_string(), Value::Int(2));
    let obj2 = Value::Object(map2);

    let result = umt_merge(&obj1, &[obj2]);

    assert_eq!(result.get("a"), Some(&Value::Int(2)));
}
