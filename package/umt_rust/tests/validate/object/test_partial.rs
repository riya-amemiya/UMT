use std::collections::HashMap;
use umt_rust::validate::object::umt_partial;

fn base_shape() -> HashMap<String, Box<dyn Fn(&i32) -> bool>> {
    let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
    shape.insert("name".to_string(), Box::new(|_v: &i32| true));
    shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
    shape
}

#[test]
fn test_covers_the_same_keys() {
    let partial = umt_partial(base_shape());
    assert_eq!(partial.len(), 2);
    assert!(partial.contains_key("name"));
    assert!(partial.contains_key("age"));
}

#[test]
fn test_passes_for_absent_values() {
    let partial = umt_partial(base_shape());
    let age = partial.get("age").unwrap();
    assert!(age(&None));
}

#[test]
fn test_delegates_to_the_wrapped_validator_for_present_values() {
    let partial = umt_partial(base_shape());
    let age = partial.get("age").unwrap();
    assert!(age(&Some(30)));
    assert!(!age(&Some(-1)));
}

#[test]
fn test_returns_an_empty_shape_for_an_empty_source() {
    let empty: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
    let partial = umt_partial(empty);
    assert!(partial.is_empty());
}
