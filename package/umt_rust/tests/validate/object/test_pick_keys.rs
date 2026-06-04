use std::collections::HashMap;
use umt_rust::validate::object::{umt_pick_keys, umt_validate_object};

fn base_shape() -> HashMap<String, Box<dyn Fn(&i32) -> bool>> {
    let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
    shape.insert("name".to_string(), Box::new(|_v: &i32| true));
    shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
    shape.insert("active".to_string(), Box::new(|_v: &i32| true));
    shape
}

#[test]
fn test_creates_shape_covering_only_picked_keys() {
    let picked = umt_pick_keys(base_shape(), &["name".to_string(), "age".to_string()]);
    assert!(picked.contains_key("name"));
    assert!(picked.contains_key("age"));
    assert!(!picked.contains_key("active"));

    let mut value: HashMap<String, i32> = HashMap::new();
    value.insert("name".to_string(), 0);
    value.insert("age".to_string(), 30);
    let result = umt_validate_object(&value, Some(&picked), None);
    assert!(result.validate);
}

#[test]
fn test_ignores_keys_outside_the_picked_set() {
    let picked = umt_pick_keys(base_shape(), &["name".to_string()]);
    assert_eq!(picked.len(), 1);
    assert!(picked.contains_key("name"));
}

#[test]
fn test_rejects_values_failing_a_picked_validator() {
    let picked = umt_pick_keys(base_shape(), &["age".to_string()]);
    let mut value: HashMap<String, i32> = HashMap::new();
    value.insert("age".to_string(), -1);
    let result = umt_validate_object(&value, Some(&picked), None);
    assert!(!result.validate);
}

#[test]
fn test_skips_requested_key_absent_from_source_shape() {
    let picked = umt_pick_keys(base_shape(), &["missing".to_string()]);
    assert!(picked.is_empty());
}
