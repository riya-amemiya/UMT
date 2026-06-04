use std::collections::HashMap;
use umt_rust::object::{Value, umt_pick_by};

#[test]
fn test_selects_entries_where_predicate_returns_true() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    map.insert("c".to_string(), Value::Int(3));

    let result = umt_pick_by(&map, |value, _| matches!(value, Value::Int(v) if *v > 1));

    let mut expected = HashMap::new();
    expected.insert("b".to_string(), Value::Int(2));
    expected.insert("c".to_string(), Value::Int(3));
    assert_eq!(result, expected);
}

#[test]
fn test_returns_empty_when_nothing_matches() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));

    let result = umt_pick_by(&map, |_, _| false);
    assert!(result.is_empty());
}

#[test]
fn test_returns_all_entries_when_everything_matches() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_pick_by(&map, |_, _| true);
    assert_eq!(result, map);
}

#[test]
fn test_passes_the_key_as_second_argument() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_pick_by(&map, |_, key| key == "a");

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Int(1));
    assert_eq!(result, expected);
}

#[test]
fn test_returns_empty_for_empty_input() {
    let map: HashMap<String, Value> = HashMap::new();
    let result = umt_pick_by(&map, |_, _| true);
    assert!(result.is_empty());
}

#[test]
fn test_does_not_mutate_the_input() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    let snapshot = map.clone();

    let _ = umt_pick_by(&map, |_, _| true);

    assert_eq!(map, snapshot);
}
