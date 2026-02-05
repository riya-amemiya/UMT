use std::collections::HashMap;
use umt_rust::object::{Value, umt_omit};

#[test]
fn test_should_omit_a_single_key() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_omit(&obj, &["a"]);

    if let Value::Object(res_map) = result {
        assert!(!res_map.contains_key("a"));
        assert!(res_map.contains_key("b"));
        assert!(res_map.contains_key("c"));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_should_omit_multiple_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_omit(&obj, &["a", "c"]);

    if let Value::Object(res_map) = result {
        assert!(!res_map.contains_key("a"));
        assert!(res_map.contains_key("b"));
        assert!(!res_map.contains_key("c"));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    let obj = Value::Object(map);

    let result = umt_omit(&obj, &["c"]);

    if let Value::Object(res_map) = result {
        assert!(res_map.contains_key("a"));
    } else {
        panic!("Expected object");
    }
}
