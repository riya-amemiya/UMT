//! Tests for is_array function

use umt_rust::validate::umt_is_array;

#[test]
fn test_is_array_returns_true_for_arrays() {
    assert!(umt_is_array(&vec![1, 2, 3]));
    assert!(umt_is_array(&vec!["a", "b", "c"]));
    assert!(umt_is_array(&vec![true, false]));
    assert!(umt_is_array(&Vec::<()>::new()));
}

#[test]
fn test_is_array_returns_true_for_empty_array() {
    let empty: Vec<i32> = Vec::new();
    assert!(umt_is_array(&empty));
}

#[test]
fn test_is_array_with_different_types() {
    // In Rust, arrays are typed, so we just verify that the function works with various types
    let int_arr = vec![1, 2, 3];
    assert!(umt_is_array(&int_arr));

    let str_arr = vec!["hello", "world"];
    assert!(umt_is_array(&str_arr));

    let bool_arr = vec![true, false, true];
    assert!(umt_is_array(&bool_arr));

    // Nested arrays
    let nested_arr = vec![vec![1, 2], vec![3, 4]];
    assert!(umt_is_array(&nested_arr));
}
