use std::collections::HashMap;
use umt_rust::object::{Value, umt_has, umt_has_path};

fn create_nested_object() -> Value {
    let mut inner = HashMap::new();
    inner.insert("b".to_string(), Value::Int(1));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(inner));
    Value::Object(obj)
}

#[test]
fn test_should_return_true_for_existing_nested_path_string() {
    let obj = create_nested_object();
    assert!(umt_has(&obj, "a.b"));
}

#[test]
fn test_should_return_true_for_existing_nested_path_array() {
    let obj = create_nested_object();
    assert!(umt_has_path(&obj, &["a", "b"]));
}

#[test]
fn test_should_return_false_for_non_existing_nested_path_string() {
    let obj = create_nested_object();
    assert!(!umt_has(&obj, "a.c"));
}

#[test]
fn test_should_return_false_for_non_existing_nested_path_array() {
    let obj = create_nested_object();
    assert!(!umt_has_path(&obj, &["a", "c"]));
}

#[test]
fn test_should_return_false_for_non_existing_top_level_path() {
    let obj = create_nested_object();
    assert!(!umt_has(&obj, "b"));
}

#[test]
fn test_should_handle_empty_path_string() {
    let obj = create_nested_object();
    assert!(!umt_has(&obj, ""));
}

#[test]
fn test_should_handle_empty_path_array() {
    let obj = create_nested_object();
    assert!(umt_has_path(&obj, &[]));
}

#[test]
fn test_should_handle_empty_object() {
    let obj = Value::Object(HashMap::new());
    assert!(!umt_has(&obj, "a.b"));
}

#[test]
fn test_should_handle_deeply_nested_paths() {
    let mut level3 = HashMap::new();
    level3.insert("d".to_string(), Value::Int(42));

    let mut level2 = HashMap::new();
    level2.insert("c".to_string(), Value::Object(level3));

    let mut level1 = HashMap::new();
    level1.insert("b".to_string(), Value::Object(level2));

    let mut obj_map = HashMap::new();
    obj_map.insert("a".to_string(), Value::Object(level1));
    let obj = Value::Object(obj_map);

    assert!(umt_has(&obj, "a.b.c.d"));
    assert!(!umt_has(&obj, "a.b.c.e"));
}

#[test]
fn test_should_return_false_when_intermediate_path_is_not_object() {
    let mut obj_map = HashMap::new();
    obj_map.insert("a".to_string(), Value::Int(1));
    let obj = Value::Object(obj_map);

    assert!(!umt_has(&obj, "a.b"));
}

#[test]
fn test_should_handle_path_with_single_key() {
    let obj = create_nested_object();
    assert!(umt_has(&obj, "a"));
    assert!(!umt_has(&obj, "c"));
}
