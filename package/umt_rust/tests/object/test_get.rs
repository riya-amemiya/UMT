use std::collections::HashMap;
use umt_rust::obj;
use umt_rust::object::{Value, umt_get, umt_get_path};

fn parts(path: &[&str]) -> Vec<String> {
    path.iter().map(|s| s.to_string()).collect()
}

#[test]
fn test_reads_a_top_level_property() {
    let data = obj! {"a" => 1};
    assert_eq!(umt_get_path(&data, "a", None), Some(Value::Int(1)));
}

#[test]
fn test_reads_a_nested_property_by_dotted_path() {
    let data = obj! {"a" => obj!{"b" => obj!{"c" => 42}}};
    assert_eq!(umt_get_path(&data, "a.b.c", None), Some(Value::Int(42)));
}

#[test]
fn test_reads_a_nested_property_by_array_path() {
    let data = obj! {"a" => obj!{"b" => obj!{"c" => 42}}};
    assert_eq!(
        umt_get(&data, &parts(&["a", "b", "c"]), None),
        Some(Value::Int(42))
    );
}

#[test]
fn test_returns_the_default_when_a_segment_is_missing() {
    let data = obj! {"a" => obj!{"b" => 1}};
    assert_eq!(
        umt_get_path(&data, "a.x", Some(Value::String("fallback".to_string()))),
        Some(Value::String("fallback".to_string()))
    );
}

#[test]
fn test_returns_the_default_when_traversing_through_null() {
    let data = obj! {"a" => Value::Null};
    assert_eq!(
        umt_get_path(&data, "a.b", Some(Value::String("fallback".to_string()))),
        Some(Value::String("fallback".to_string()))
    );
}

#[test]
fn test_returns_none_when_no_default_and_missing() {
    let data = obj! {"a" => 1};
    assert_eq!(umt_get_path(&data, "x", None), None);
}

#[test]
fn test_returns_null_when_the_value_is_explicitly_null() {
    let data = obj! {"a" => Value::Null};
    assert_eq!(
        umt_get_path(&data, "a", Some(Value::String("fallback".to_string()))),
        Some(Value::Null)
    );
}

#[test]
fn test_handles_object_root_being_null() {
    let root = Value::Null;
    assert_eq!(
        umt_get_path(&root, "a.b", Some(Value::Int(7))),
        Some(Value::Int(7))
    );
}

#[test]
fn test_returns_default_when_indexing_into_primitive() {
    let data = obj! {"a" => 1};
    assert_eq!(
        umt_get_path(&data, "a.b", Some(Value::Int(0))),
        Some(Value::Int(0))
    );
}
