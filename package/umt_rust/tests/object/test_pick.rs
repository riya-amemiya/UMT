use std::collections::HashMap;
use umt_rust::object::{umt_pick, umt_pick_string_keys, Value};

#[test]
fn test_should_select_a_single_key() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick(&obj, &["a"]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
}

#[test]
fn test_should_select_multiple_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick(&obj, &["a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_pick(&obj, &["c"]);

    assert!(result.is_empty());
}

#[test]
fn test_should_handle_empty_objects() {
    let obj: HashMap<String, Value> = HashMap::new();
    let result = umt_pick(&obj, &["a"]);
    assert!(result.is_empty());
}

#[test]
fn test_should_handle_no_keys_specified() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_pick(&obj, &[]);

    assert!(result.is_empty());
}

#[test]
fn test_should_select_all_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick(&obj, &["a", "b", "c"]);

    assert_eq!(result.len(), 3);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_duplicate_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick(&obj, &["a", "a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_nested_objects() {
    let mut nested = HashMap::new();
    nested.insert("b".to_string(), Value::Int(1));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(nested.clone()));
    obj.insert("c".to_string(), Value::Int(2));

    let result = umt_pick(&obj, &["a"]);

    assert_eq!(result.len(), 1);
    assert_eq!(result.get("a"), Some(&Value::Object(nested)));
}

#[test]
fn test_should_handle_objects_with_null_properties() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Null);
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_pick(&obj, &["a", "c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Null));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_objects_containing_arrays() {
    let mut obj = HashMap::new();
    obj.insert(
        "a".to_string(),
        Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
    );
    obj.insert("b".to_string(), Value::Int(4));

    let result = umt_pick(&obj, &["a"]);

    assert_eq!(result.len(), 1);
    assert_eq!(
        result.get("a"),
        Some(&Value::Array(vec![
            Value::Int(1),
            Value::Int(2),
            Value::Int(3)
        ]))
    );
}

#[test]
fn test_pick_with_string_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let keys = vec!["a".to_string(), "c".to_string()];
    let result = umt_pick_string_keys(&obj, &keys);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}
