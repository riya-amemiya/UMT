use std::collections::HashMap;
use umt_rust::object::{Value, umt_omit_by};

#[test]
fn test_removes_entries_where_predicate_returns_true() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Null);
    map.insert("c".to_string(), Value::Int(3));

    let result = umt_omit_by(&map, |value, _| value.is_null());

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Int(1));
    expected.insert("c".to_string(), Value::Int(3));
    assert_eq!(result, expected);
}

#[test]
fn test_returns_all_entries_when_predicate_always_false() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_omit_by(&map, |_, _| false);
    assert_eq!(result, map);
}

#[test]
fn test_returns_empty_when_predicate_always_true() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_omit_by(&map, |_, _| true);
    assert!(result.is_empty());
}

#[test]
fn test_passes_the_key_as_second_argument() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_omit_by(&map, |_, key| key == "a");

    let mut expected = HashMap::new();
    expected.insert("b".to_string(), Value::Int(2));
    assert_eq!(result, expected);
}

#[test]
fn test_returns_empty_for_empty_input() {
    let map: HashMap<String, Value> = HashMap::new();
    let result = umt_omit_by(&map, |_, _| false);
    assert!(result.is_empty());
}
