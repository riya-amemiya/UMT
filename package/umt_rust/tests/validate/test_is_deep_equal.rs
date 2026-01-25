//! Tests for is_deep_equal function

use std::collections::{HashMap, HashSet};
use umt_rust::validate::{
    umt_is_deep_equal, umt_is_deep_equal_map, umt_is_deep_equal_set, IsDeepEqualOptions,
};

// Basic array comparison tests
#[test]
fn test_is_deep_equal_arrays_with_same_elements() {
    assert!(umt_is_deep_equal(&vec![1, 2, 3], &vec![1, 2, 3], None));
    assert!(umt_is_deep_equal(
        &Vec::<i32>::new(),
        &Vec::<i32>::new(),
        None
    ));
    assert!(umt_is_deep_equal(
        &vec![vec![1, 2], vec![3, 4]],
        &vec![vec![1, 2], vec![3, 4]],
        None
    ));
}

#[test]
fn test_is_deep_equal_arrays_different() {
    assert!(!umt_is_deep_equal(&vec![1, 2, 3], &vec![1, 2, 4], None));
    assert!(!umt_is_deep_equal(&vec![1, 2, 3], &vec![1, 2, 3, 4], None));
    assert!(!umt_is_deep_equal(&vec![1, 2, 3], &vec![3, 2, 1], None));
}

#[test]
fn test_is_deep_equal_strict_order_option() {
    let opts_strict = IsDeepEqualOptions { strict_order: true };
    assert!(!umt_is_deep_equal(
        &vec![1, 2, 3],
        &vec![3, 2, 1],
        Some(opts_strict)
    ));

    let opts_not_strict = IsDeepEqualOptions {
        strict_order: false,
    };
    assert!(umt_is_deep_equal(
        &vec![1, 2, 3],
        &vec![3, 2, 1],
        Some(opts_not_strict)
    ));
}

#[test]
fn test_is_deep_equal_empty_arrays() {
    assert!(umt_is_deep_equal(
        &Vec::<i32>::new(),
        &Vec::<i32>::new(),
        None
    ));
}

#[test]
fn test_is_deep_equal_unordered_no_match() {
    let opts = IsDeepEqualOptions {
        strict_order: false,
    };
    assert!(!umt_is_deep_equal(
        &vec![1, 2, 3],
        &vec![1, 2, 4],
        Some(opts)
    ));
}

// HashMap comparison tests
#[test]
fn test_is_deep_equal_map_same() {
    let mut a: HashMap<&str, i32> = HashMap::new();
    a.insert("a", 1);
    a.insert("b", 2);

    let mut b: HashMap<&str, i32> = HashMap::new();
    b.insert("a", 1);
    b.insert("b", 2);

    assert!(umt_is_deep_equal_map(&a, &b));
}

#[test]
fn test_is_deep_equal_map_different_values() {
    let mut a: HashMap<&str, i32> = HashMap::new();
    a.insert("a", 1);
    a.insert("b", 2);

    let mut b: HashMap<&str, i32> = HashMap::new();
    b.insert("a", 1);
    b.insert("b", 3);

    assert!(!umt_is_deep_equal_map(&a, &b));
}

#[test]
fn test_is_deep_equal_map_different_sizes() {
    let mut a: HashMap<&str, i32> = HashMap::new();
    a.insert("a", 1);

    let mut b: HashMap<&str, i32> = HashMap::new();
    b.insert("a", 1);
    b.insert("b", 2);

    assert!(!umt_is_deep_equal_map(&a, &b));
}

#[test]
fn test_is_deep_equal_empty_maps() {
    let a: HashMap<&str, i32> = HashMap::new();
    let b: HashMap<&str, i32> = HashMap::new();
    assert!(umt_is_deep_equal_map(&a, &b));
}

// HashSet comparison tests
#[test]
fn test_is_deep_equal_set_same() {
    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = [3, 2, 1].into_iter().collect();
    assert!(umt_is_deep_equal_set(&a, &b));
}

#[test]
fn test_is_deep_equal_set_different() {
    let a: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let b: HashSet<i32> = [1, 2, 4].into_iter().collect();
    assert!(!umt_is_deep_equal_set(&a, &b));
}

#[test]
fn test_is_deep_equal_empty_sets() {
    let a: HashSet<i32> = HashSet::new();
    let b: HashSet<i32> = HashSet::new();
    assert!(umt_is_deep_equal_set(&a, &b));
}

// String array tests
#[test]
fn test_is_deep_equal_string_arrays() {
    assert!(umt_is_deep_equal(
        &vec!["a", "b", "c"],
        &vec!["a", "b", "c"],
        None
    ));
    assert!(!umt_is_deep_equal(
        &vec!["a", "b", "c"],
        &vec!["c", "b", "a"],
        None
    ));
}
