use std::collections::HashMap;
use umt_rust::object::{umt_omit, umt_omit_string_keys, Value};

#[test]
fn test_should_omit_specified_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));
    obj.insert("d".to_string(), Value::Int(4));

    let result = umt_omit(&obj, &["b", "d"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
    assert!(result.get("b").is_none());
    assert!(result.get("d").is_none());
}

#[test]
fn test_should_not_modify_original_object() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let obj_clone = obj.clone();
    let result = umt_omit(&obj, &["b"]);

    assert_eq!(obj, obj_clone);
    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_non_existent_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_omit(&obj, &["c"]);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_should_handle_empty_object() {
    let obj: HashMap<String, Value> = HashMap::new();
    let result = umt_omit(&obj, &["a"]);
    assert!(result.is_empty());
}

#[test]
fn test_should_handle_no_keys_to_omit() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let result = umt_omit(&obj, &[]);

    assert_eq!(result.len(), 3);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("b"), Some(&Value::Int(2)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
}

#[test]
fn test_should_handle_omitting_all_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));

    let result = umt_omit(&obj, &["a", "b"]);

    assert!(result.is_empty());
}

#[test]
fn test_should_handle_various_value_types() {
    let mut nested = HashMap::new();
    nested.insert("nested".to_string(), Value::Bool(true));

    let mut obj = HashMap::new();
    obj.insert("string".to_string(), Value::String("test".to_string()));
    obj.insert("number".to_string(), Value::Int(42));
    obj.insert("boolean".to_string(), Value::Bool(true));
    obj.insert(
        "array".to_string(),
        Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]),
    );
    obj.insert("object".to_string(), Value::Object(nested.clone()));
    obj.insert("null".to_string(), Value::Null);

    let result = umt_omit(&obj, &["string", "array", "null"]);

    assert_eq!(result.len(), 3);
    assert_eq!(result.get("number"), Some(&Value::Int(42)));
    assert_eq!(result.get("boolean"), Some(&Value::Bool(true)));
    assert_eq!(result.get("object"), Some(&Value::Object(nested)));
    assert!(result.get("string").is_none());
    assert!(result.get("array").is_none());
    assert!(result.get("null").is_none());
}

#[test]
fn test_omit_with_string_keys() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));
    obj.insert("b".to_string(), Value::Int(2));
    obj.insert("c".to_string(), Value::Int(3));

    let keys = vec!["b".to_string()];
    let result = umt_omit_string_keys(&obj, &keys);

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("a"), Some(&Value::Int(1)));
    assert_eq!(result.get("c"), Some(&Value::Int(3)));
    assert_eq!(result.get("b"), None);
}
