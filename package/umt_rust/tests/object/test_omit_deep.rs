use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_omit_deep, umt_omit_deep_string_keys};

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

#[test]
fn test_removes_simple_keys() {
    let source = as_map(obj! {"a" => 1, "b" => 2, "c" => 3});
    assert_eq!(
        Value::Object(umt_omit_deep(&source, &["a", "c"])),
        obj! {"b" => 2}
    );
}

#[test]
fn test_removes_nested_keys() {
    let source = as_map(obj! {
        "a" => obj!{"b" => obj!{"c" => 1, "d" => 2}, "e" => 3},
        "f" => 4
    });
    assert_eq!(
        Value::Object(umt_omit_deep(&source, &["a.b.c", "f"])),
        obj! {"a" => obj!{"b" => obj!{"d" => 2}, "e" => 3}}
    );
}

#[test]
fn test_does_not_mutate_the_input() {
    let source = as_map(obj! {"a" => obj!{"b" => 1, "c" => 2}});
    let before = source.clone();
    umt_omit_deep(&source, &["a.b"]);
    assert_eq!(source, before);
}

#[test]
fn test_returns_a_copy_when_no_keys_are_given() {
    let source = as_map(obj! {"a" => 1});
    assert_eq!(umt_omit_deep(&source, &[]), source);
}

#[test]
fn test_ignores_missing_keys() {
    let source = as_map(obj! {"a" => obj!{"b" => 1}});
    assert_eq!(umt_omit_deep(&source, &["a.x", "z"]), source);
}

#[test]
fn test_removes_keys_from_objects_inside_arrays() {
    let source = as_map(obj! {"a" => vec![obj!{"b" => 1, "c" => 2}]});
    assert_eq!(
        Value::Object(umt_omit_deep(&source, &["a.0.b"])),
        obj! {"a" => vec![obj!{"c" => 2}]}
    );
}

#[test]
fn test_accepts_string_keys() {
    let source = as_map(obj! {"a" => obj!{"b" => 1, "c" => 2}});
    let keys = vec!["a.b".to_string()];
    assert_eq!(
        Value::Object(umt_omit_deep_string_keys(&source, &keys)),
        obj! {"a" => obj!{"c" => 2}}
    );
}
