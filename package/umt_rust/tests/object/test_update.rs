use std::cell::RefCell;
use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_update, umt_update_path};

fn parts(path: &[&str]) -> Vec<String> {
    path.iter().map(|s| s.to_string()).collect()
}

fn as_map(value: Value) -> HashMap<String, Value> {
    value.into_object().unwrap()
}

fn increment(value: Option<Value>) -> Value {
    match value {
        Some(Value::Int(n)) => Value::Int(n + 1),
        _ => Value::Null,
    }
}

#[test]
fn test_updates_a_top_level_property() {
    let mut target = as_map(obj! {"a" => 1});
    umt_update_path(&mut target, "a", increment);
    assert_eq!(Value::Object(target), obj! {"a" => 2});
}

#[test]
fn test_updates_a_nested_property() {
    let mut target = as_map(obj! {"a" => obj!{"b" => 1}});
    umt_update_path(&mut target, "a.b", increment);
    assert_eq!(Value::Object(target), obj! {"a" => obj!{"b" => 2}});
}

#[test]
fn test_accepts_a_segment_path() {
    let mut target = as_map(obj! {"a" => obj!{"b" => 1}});
    umt_update(&mut target, &parts(&["a", "b"]), increment);
    assert_eq!(Value::Object(target), obj! {"a" => obj!{"b" => 2}});
}

#[test]
fn test_passes_none_and_creates_intermediates_when_missing() {
    let received = RefCell::new(Vec::new());
    let mut target: HashMap<String, Value> = HashMap::new();
    umt_update_path(&mut target, "a.b", |value| {
        received.borrow_mut().push(value);
        Value::Int(0)
    });
    assert_eq!(*received.borrow(), vec![None]);
    assert_eq!(Value::Object(target), obj! {"a" => obj!{"b" => 0}});
}

#[test]
fn test_passes_explicit_null() {
    let received = RefCell::new(Vec::new());
    let mut target = as_map(obj! {"a" => Value::Null});
    umt_update_path(&mut target, "a", |value| {
        received.borrow_mut().push(value);
        Value::Int(1)
    });
    assert_eq!(*received.borrow(), vec![Some(Value::Null)]);
    assert_eq!(Value::Object(target), obj! {"a" => 1});
}

#[test]
fn test_does_not_call_the_updater_for_an_empty_path() {
    let called = RefCell::new(false);
    let mut target = as_map(obj! {"a" => 1});
    umt_update(&mut target, &[], |value| {
        *called.borrow_mut() = true;
        value.unwrap_or_default()
    });
    assert!(!*called.borrow());
    assert_eq!(Value::Object(target), obj! {"a" => 1});
}

#[test]
fn test_replaces_non_object_intermediate_values() {
    let mut target = as_map(obj! {"a" => 5});
    umt_update_path(&mut target, "a.b", |_| Value::Int(1));
    assert_eq!(Value::Object(target), obj! {"a" => obj!{"b" => 1}});
}
