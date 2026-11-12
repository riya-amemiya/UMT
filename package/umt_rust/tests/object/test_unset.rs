use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_unset, umt_unset_path};

fn parts(path: &[&str]) -> Vec<String> {
    path.iter().map(|s| s.to_string()).collect()
}

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

fn ints(values: &[i64]) -> Value {
    Value::Array(values.iter().map(|value| Value::Int(*value)).collect())
}

#[test]
fn test_removes_a_top_level_property() {
    let source = as_map(obj! {"a" => 1, "b" => 2});
    assert_eq!(Value::Object(umt_unset_path(&source, "a")), obj! {"b" => 2});
}

#[test]
fn test_removes_a_nested_property() {
    let source = as_map(obj! {"a" => obj!{"b" => 1, "c" => 2}});
    assert_eq!(
        Value::Object(umt_unset_path(&source, "a.b")),
        obj! {"a" => obj!{"c" => 2}}
    );
}

#[test]
fn test_accepts_a_segment_path() {
    let source = as_map(obj! {"a" => obj!{"b" => 1, "c" => 2}});
    assert_eq!(
        Value::Object(umt_unset(&source, &parts(&["a", "c"]))),
        obj! {"a" => obj!{"b" => 1}}
    );
}

#[test]
fn test_does_not_mutate_the_input() {
    let source = as_map(obj! {"a" => obj!{"b" => 1, "c" => 2}, "d" => obj!{"e" => 3}});
    let before = source.clone();
    let result = umt_unset_path(&source, "a.b");
    assert_eq!(source, before);
    assert_eq!(result.get("d"), source.get("d"));
}

#[test]
fn test_returns_a_copy_when_the_path_is_missing() {
    let source = as_map(obj! {"a" => obj!{"b" => 1}});
    assert_eq!(umt_unset_path(&source, "a.x.y"), source);
}

#[test]
fn test_returns_a_copy_when_the_path_is_empty() {
    let source = as_map(obj! {"a" => 1});
    assert_eq!(umt_unset(&source, &[]), source);
}

#[test]
fn test_removes_an_array_element_and_shifts_the_rest() {
    let source = as_map(obj! {"a" => ints(&[1, 2, 3])});
    let result = umt_unset_path(&source, "a.1");
    assert_eq!(result.get("a"), Some(&ints(&[1, 3])));
    assert_eq!(source.get("a"), Some(&ints(&[1, 2, 3])));
}

#[test]
fn test_traverses_through_array_elements() {
    let source = as_map(obj! {"a" => vec![obj!{"b" => 1, "c" => 2}]});
    assert_eq!(
        Value::Object(umt_unset_path(&source, "a.0.b")),
        obj! {"a" => vec![obj!{"c" => 2}]}
    );
}

#[test]
fn test_ignores_non_canonical_array_indexes() {
    let source = as_map(obj! {"a" => ints(&[1, 2])});
    assert_eq!(umt_unset_path(&source, "a.01"), source);
    assert_eq!(umt_unset_path(&source, "a.length"), source);
}

#[test]
fn test_ignores_out_of_range_array_indexes() {
    let source = as_map(obj! {"a" => ints(&[1, 2])});
    assert_eq!(umt_unset_path(&source, "a.5"), source);
}

#[test]
fn test_stops_at_primitive_values() {
    let source = as_map(obj! {"a" => "text"});
    assert_eq!(umt_unset_path(&source, "a.length"), source);
    let numbers = as_map(obj! {"a" => ints(&[1, 2])});
    assert_eq!(umt_unset_path(&numbers, "a.0.x"), numbers);
}
