//! Tests for order-insensitive is_deep_equal comparison

use umt_rust::validate::{IsDeepEqualOptions, umt_is_deep_equal};

fn unordered() -> Option<IsDeepEqualOptions> {
    Some(IsDeepEqualOptions {
        strict_order: false,
    })
}

#[test]
fn test_is_deep_equal_unordered_matches_permutations() {
    assert!(umt_is_deep_equal(
        &vec![1, 2, 3],
        &vec![3, 1, 2],
        unordered()
    ));
    assert!(umt_is_deep_equal(
        &vec!["a", "b"],
        &vec!["b", "a"],
        unordered()
    ));
    assert!(umt_is_deep_equal(
        &Vec::<i32>::new(),
        &Vec::<i32>::new(),
        unordered()
    ));
}

#[test]
fn test_is_deep_equal_unordered_respects_multiplicity() {
    assert!(umt_is_deep_equal(
        &vec![1, 1, 2],
        &vec![1, 2, 1],
        unordered()
    ));
    assert!(!umt_is_deep_equal(
        &vec![1, 1, 2],
        &vec![1, 2, 2],
        unordered()
    ));
    assert!(!umt_is_deep_equal(
        &vec![1, 2, 2],
        &vec![1, 1, 2],
        unordered()
    ));
    assert!(!umt_is_deep_equal(&vec![1, 1], &vec![2, 3], unordered()));
}

#[test]
fn test_is_deep_equal_unordered_rejects_different_lengths() {
    assert!(!umt_is_deep_equal(&vec![1, 2], &vec![1, 2, 2], unordered()));
    assert!(!umt_is_deep_equal(
        &vec![1],
        &Vec::<i32>::new(),
        unordered()
    ));
}

#[test]
fn test_is_deep_equal_unordered_rejects_missing_elements() {
    assert!(!umt_is_deep_equal(
        &vec![1, 2, 3],
        &vec![1, 2, 4],
        unordered()
    ));
}

#[test]
fn test_is_deep_equal_unordered_never_matches_nan() {
    assert!(!umt_is_deep_equal(
        &vec![f64::NAN],
        &vec![f64::NAN],
        unordered()
    ));
}

#[test]
fn test_is_deep_equal_unordered_handles_nested_vectors() {
    assert!(umt_is_deep_equal(
        &vec![vec![1, 2], vec![3]],
        &vec![vec![3], vec![1, 2]],
        unordered()
    ));
    assert!(!umt_is_deep_equal(
        &vec![vec![1, 2], vec![3]],
        &vec![vec![3], vec![2, 1]],
        unordered()
    ));
}

#[test]
fn test_is_deep_equal_unordered_leaves_inputs_untouched() {
    let a = vec![3, 1, 2];
    let b = vec![1, 2, 3];
    assert!(umt_is_deep_equal(&a, &b, unordered()));
    assert_eq!(a, vec![3, 1, 2]);
    assert_eq!(b, vec![1, 2, 3]);
}
