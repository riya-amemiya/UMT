use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_flatten_object};

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

#[test]
fn test_flattens_a_nested_object_using_dot_separator() {
    let map = as_map(obj! {"a" => obj!{"b" => obj!{"c" => 1}}});
    let flat = umt_flatten_object(&map, ".");

    let mut expected = HashMap::new();
    expected.insert("a.b.c".to_string(), Value::Int(1));
    assert_eq!(flat, expected);
}

#[test]
fn test_preserves_top_level_keys() {
    let map = as_map(obj! {"a" => 1, "b" => 2});
    let flat = umt_flatten_object(&map, ".");

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Int(1));
    expected.insert("b".to_string(), Value::Int(2));
    assert_eq!(flat, expected);
}

#[test]
fn test_supports_custom_separator() {
    let map = as_map(obj! {"a" => obj!{"b" => 1}});
    let flat = umt_flatten_object(&map, "/");

    let mut expected = HashMap::new();
    expected.insert("a/b".to_string(), Value::Int(1));
    assert_eq!(flat, expected);
}

#[test]
fn test_keeps_arrays_as_values() {
    let map = as_map(obj! {"a" => obj!{"b" => Value::Array(vec![Value::Int(1), Value::Int(2)])}});
    let flat = umt_flatten_object(&map, ".");

    let mut expected = HashMap::new();
    expected.insert(
        "a.b".to_string(),
        Value::Array(vec![Value::Int(1), Value::Int(2)]),
    );
    assert_eq!(flat, expected);
}

#[test]
fn test_treats_empty_plain_objects_as_leaf_values() {
    let map = as_map(obj! {"a" => Value::Object(HashMap::new())});
    let flat = umt_flatten_object(&map, ".");

    let mut expected = HashMap::new();
    expected.insert("a".to_string(), Value::Object(HashMap::new()));
    assert_eq!(flat, expected);
}

#[test]
fn test_returns_empty_for_empty_input() {
    let map: HashMap<String, Value> = HashMap::new();
    let flat = umt_flatten_object(&map, ".");
    assert!(flat.is_empty());
}
