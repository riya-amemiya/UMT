use std::collections::HashMap;
use umt_rust::object::{Value, umt_has, umt_has_path};

fn create_nested_object() -> HashMap<String, Value> {
    let mut inner = HashMap::new();
    inner.insert("b".to_string(), Value::Int(1));

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(inner));
    obj
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
    let obj: HashMap<String, Value> = HashMap::new();
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

    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Object(level1));

    assert!(umt_has(&obj, "a.b.c.d"));
    assert!(!umt_has(&obj, "a.b.c.e"));
}

#[test]
fn test_should_return_false_when_intermediate_path_is_not_object() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Int(1));

    assert!(!umt_has(&obj, "a.b"));
}

#[test]
fn test_should_handle_path_with_single_key() {
    let obj = create_nested_object();
    assert!(umt_has(&obj, "a"));
    assert!(!umt_has(&obj, "c"));
}

fn create_object_with_list() -> HashMap<String, Value> {
    let mut element = HashMap::new();
    element.insert("a".to_string(), Value::Int(1));

    let mut obj = HashMap::new();
    obj.insert(
        "list".to_string(),
        Value::Array(vec![Value::Int(10), Value::Object(element)]),
    );
    obj
}

#[test]
fn test_should_match_array_indices_as_own_keys() {
    let obj = create_object_with_list();
    assert!(umt_has(&obj, "list.0"));
    assert!(umt_has(&obj, "list.1"));
    assert!(!umt_has(&obj, "list.2"));
    assert!(!umt_has(&obj, "list.push"));
}

#[test]
fn test_should_reject_non_canonical_array_indices() {
    let obj = create_object_with_list();
    assert!(!umt_has(&obj, "list.00"));
    assert!(!umt_has(&obj, "list.01"));
    assert!(!umt_has(&obj, "list.+1"));
    assert!(!umt_has(&obj, "list.-1"));
    assert!(!umt_has_path(&obj, &["list", ""]));
    assert!(!umt_has(&obj, "list.99999999999999999999999999"));
}

#[test]
fn test_should_traverse_objects_inside_arrays() {
    let obj = create_object_with_list();
    assert!(umt_has(&obj, "list.1.a"));
    assert!(!umt_has(&obj, "list.1.b"));
    assert!(!umt_has(&obj, "list.0.a"));
}

#[test]
fn test_should_report_an_own_empty_string_key() {
    let mut inner = HashMap::new();
    inner.insert("".to_string(), Value::Int(1));

    let mut obj = HashMap::new();
    obj.insert("".to_string(), Value::Object(inner));

    assert!(umt_has(&obj, ""));
    assert!(umt_has(&obj, "."));
    assert!(!umt_has(&obj, ".."));
    assert!(umt_has_path(&obj, &[""]));
}

#[test]
fn test_should_return_false_when_traversing_through_null() {
    let mut obj = HashMap::new();
    obj.insert("a".to_string(), Value::Null);

    assert!(umt_has(&obj, "a"));
    assert!(!umt_has(&obj, "a.b"));
}
