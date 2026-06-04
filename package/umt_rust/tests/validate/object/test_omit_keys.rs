use std::collections::HashMap;
use umt_rust::validate::object::{umt_omit_keys, umt_validate_object};

fn base_shape() -> HashMap<String, Box<dyn Fn(&i32) -> bool>> {
    let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
    shape.insert("name".to_string(), Box::new(|_v: &i32| true));
    shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
    shape.insert("active".to_string(), Box::new(|_v: &i32| true));
    shape
}

#[test]
fn test_creates_shape_covering_only_remaining_keys() {
    let omitted = umt_omit_keys(base_shape(), &["active".to_string()]);
    assert!(omitted.contains_key("name"));
    assert!(omitted.contains_key("age"));
    assert!(!omitted.contains_key("active"));

    let mut value: HashMap<String, i32> = HashMap::new();
    value.insert("name".to_string(), 0);
    value.insert("age".to_string(), 30);
    let result = umt_validate_object(&value, Some(&omitted), None);
    assert!(result.validate);
}

#[test]
fn test_ignores_keys_outside_the_omitted_set() {
    let omitted = umt_omit_keys(base_shape(), &["age".to_string(), "active".to_string()]);
    assert_eq!(omitted.len(), 1);
    assert!(omitted.contains_key("name"));
}

#[test]
fn test_rejects_values_failing_a_remaining_validator() {
    let omitted = umt_omit_keys(base_shape(), &["active".to_string()]);
    let mut value: HashMap<String, i32> = HashMap::new();
    value.insert("name".to_string(), 0);
    value.insert("age".to_string(), -1);
    let result = umt_validate_object(&value, Some(&omitted), None);
    assert!(!result.validate);
}

#[test]
fn test_skips_requested_key_absent_from_source_shape() {
    let omitted = umt_omit_keys(base_shape(), &["missing".to_string()]);
    assert_eq!(omitted.len(), 3);
    assert!(omitted.contains_key("name"));
    assert!(omitted.contains_key("age"));
    assert!(omitted.contains_key("active"));
}
