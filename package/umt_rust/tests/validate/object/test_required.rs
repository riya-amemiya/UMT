use std::collections::HashMap;
use umt_rust::validate::object::{umt_partial, umt_required};

fn base_shape() -> HashMap<String, Box<dyn Fn(&i32) -> bool>> {
    let mut shape: HashMap<String, Box<dyn Fn(&i32) -> bool>> = HashMap::new();
    shape.insert("name".to_string(), Box::new(|_v: &i32| true));
    shape.insert("age".to_string(), Box::new(|v: &i32| *v >= 0));
    shape
}

#[test]
fn test_covers_the_same_keys() {
    let required = umt_required(umt_partial(base_shape()));
    assert_eq!(required.len(), 2);
    assert!(required.contains_key("name"));
    assert!(required.contains_key("age"));
}

#[test]
fn test_undoes_a_partial_wrapper() {
    let required = umt_required(umt_partial(base_shape()));
    let age = required.get("age").unwrap();
    assert!(age(&30));
    assert!(!age(&-1));
}

#[test]
fn test_delegates_present_values_to_the_inner_validator() {
    let mut shape: HashMap<String, Box<dyn Fn(&Option<i32>) -> bool>> = HashMap::new();
    shape.insert(
        "score".to_string(),
        Box::new(|value: &Option<i32>| match value {
            None => true,
            Some(inner) => *inner <= 100,
        }),
    );
    let required = umt_required(shape);
    let score = required.get("score").unwrap();
    assert!(score(&100));
    assert!(!score(&101));
}

#[test]
fn test_returns_an_empty_shape_for_an_empty_source() {
    let empty: HashMap<String, Box<dyn Fn(&Option<i32>) -> bool>> = HashMap::new();
    let required = umt_required(empty);
    assert!(required.is_empty());
}
