use std::collections::HashMap;
use umt_rust::object::{Value, umt_invert};

#[test]
fn test_swaps_keys_and_values() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));

    let result = umt_invert(&map);

    let mut expected = HashMap::new();
    expected.insert("1".to_string(), Value::String("a".to_string()));
    expected.insert("2".to_string(), Value::String("b".to_string()));
    assert_eq!(result, expected);
}

#[test]
fn test_handles_string_to_string_swap() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), Value::String("X".to_string()));
    map.insert("y".to_string(), Value::String("Y".to_string()));

    let result = umt_invert(&map);

    let mut expected = HashMap::new();
    expected.insert("X".to_string(), Value::String("x".to_string()));
    expected.insert("Y".to_string(), Value::String("y".to_string()));
    assert_eq!(result, expected);
}

#[test]
fn test_returns_empty_object_for_empty_input() {
    let map: HashMap<String, Value> = HashMap::new();
    let result = umt_invert(&map);
    assert!(result.is_empty());
}

#[test]
fn test_overwrites_earlier_keys_when_values_collide() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(1));

    let result = umt_invert(&map);

    // Both keys collide on "1"; iteration order is unspecified, so the winner
    // is either "a" or "b", but there is exactly one entry.
    assert_eq!(result.len(), 1);
    let winner = result.get("1").unwrap();
    assert!(winner == &Value::String("a".to_string()) || winner == &Value::String("b".to_string()));
}

#[test]
fn test_does_not_mutate_the_input() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), Value::Int(1));
    map.insert("b".to_string(), Value::Int(2));
    let snapshot = map.clone();

    let _ = umt_invert(&map);

    assert_eq!(map, snapshot);
}
