use std::collections::HashMap;
use umt_rust::object::{Value, umt_set, umt_set_path};

fn parts(path: &[&str]) -> Vec<String> {
    path.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_sets_a_top_level_property() {
    let mut target: HashMap<String, Value> = HashMap::new();
    umt_set_path(&mut target, "a", Value::Int(1));
    assert_eq!(target.get("a"), Some(&Value::Int(1)));
    assert_eq!(target.len(), 1);
}

#[test]
fn test_creates_nested_objects_when_missing() {
    let mut target: HashMap<String, Value> = HashMap::new();
    umt_set_path(&mut target, "a.b.c", Value::Int(1));

    let a = target.get("a").and_then(Value::as_object).unwrap();
    let b = a.get("b").and_then(Value::as_object).unwrap();
    assert_eq!(b.get("c"), Some(&Value::Int(1)));
}

#[test]
fn test_respects_array_path_input() {
    let mut target: HashMap<String, Value> = HashMap::new();
    umt_set(&mut target, &parts(&["a", "b"]), Value::Int(2));

    let a = target.get("a").and_then(Value::as_object).unwrap();
    assert_eq!(a.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_overwrites_existing_leaf_values() {
    let mut inner = HashMap::new();
    inner.insert("b".to_string(), Value::Int(1));
    let mut target: HashMap<String, Value> = HashMap::new();
    target.insert("a".to_string(), Value::Object(inner));

    umt_set_path(&mut target, "a.b", Value::Int(2));

    let a = target.get("a").and_then(Value::as_object).unwrap();
    assert_eq!(a.get("b"), Some(&Value::Int(2)));
}

#[test]
fn test_replaces_non_object_intermediate_values() {
    let mut target: HashMap<String, Value> = HashMap::new();
    target.insert("a".to_string(), Value::Int(5));

    umt_set_path(&mut target, "a.b", Value::Int(1));

    let a = target.get("a").and_then(Value::as_object).unwrap();
    assert_eq!(a.get("b"), Some(&Value::Int(1)));
}

#[test]
fn test_returns_the_object_unchanged_when_path_is_empty() {
    let mut target: HashMap<String, Value> = HashMap::new();
    target.insert("a".to_string(), Value::Int(1));
    let snapshot = target.clone();

    umt_set(&mut target, &[], Value::Int(2));

    assert_eq!(target, snapshot);
}
