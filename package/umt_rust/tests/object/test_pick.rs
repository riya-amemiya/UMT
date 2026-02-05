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

    if let Value::Object(res_map) = result {
        assert!(res_map.contains_key("a"));
        assert!(!res_map.contains_key("b"));
        assert!(!res_map.contains_key("c"));
        assert_eq!(res_map.get("a"), Some(&Value::Int(1)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_should_select_multiple_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));
    let obj = Value::Object(map);

    let result = umt_pick(&obj, &["a", "c"]);

    if let Value::Object(res_map) = result {
        assert!(res_map.contains_key("a"));
        assert!(!res_map.contains_key("b"));
        assert!(res_map.contains_key("c"));
        assert_eq!(res_map.get("a"), Some(&Value::Int(1)));
        assert_eq!(res_map.get("c"), Some(&Value::Int(3)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    let obj = Value::Object(map);

    let result = umt_pick(&obj, &["c"]);

    if let Value::Object(res_map) = result {
        assert!(res_map.is_empty());
    } else {
        panic!("Expected object");
    }
}
