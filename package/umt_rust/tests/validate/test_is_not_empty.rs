//! Tests for is_not_empty function

use std::collections::HashMap;
use umt_rust::validate::{umt_is_not_empty, umt_is_not_empty_str, umt_is_not_empty_vec};

#[test]
fn test_is_not_empty_returns_true_for_non_empty_map() {
    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("a", 1);
    assert!(umt_is_not_empty(&map));

    let mut map2: HashMap<&str, i32> = HashMap::new();
    map2.insert("a", 1);
    map2.insert("b", 2);
    assert!(umt_is_not_empty(&map2));
}

#[test]
fn test_is_not_empty_returns_false_for_empty_map() {
    let map: HashMap<&str, i32> = HashMap::new();
    assert!(!umt_is_not_empty(&map));
}

#[test]
fn test_is_not_empty_vec_returns_true_for_non_empty_array() {
    assert!(umt_is_not_empty_vec(&vec![1, 2, 3]));
    assert!(umt_is_not_empty_vec(&vec!["a", "b", "c"]));
    assert!(umt_is_not_empty_vec(&vec![true, false]));
}

#[test]
fn test_is_not_empty_vec_returns_false_for_empty_array() {
    let empty: Vec<i32> = Vec::new();
    assert!(!umt_is_not_empty_vec(&empty));
}

#[test]
fn test_is_not_empty_str_returns_true_for_non_empty_string() {
    assert!(umt_is_not_empty_str("hello"));
    assert!(umt_is_not_empty_str("a"));
    assert!(umt_is_not_empty_str(" "));
}

#[test]
fn test_is_not_empty_str_returns_false_for_empty_string() {
    assert!(!umt_is_not_empty_str(""));
}

#[test]
fn test_is_not_empty_nested_objects() {
    let mut nested: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let mut inner: HashMap<&str, i32> = HashMap::new();
    inner.insert("b", 2);
    nested.insert("a", inner);
    assert!(umt_is_not_empty(&nested));
}
