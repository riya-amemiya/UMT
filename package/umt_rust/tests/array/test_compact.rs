use umt_rust::array::{umt_compact, umt_compact_options};

#[test]
fn test_compact_removes_zeros() {
    assert_eq!(umt_compact(&[0, 1, 0, 2, 0, 3]), vec![1, 2, 3]);
}

#[test]
fn test_compact_all_truthy() {
    assert_eq!(umt_compact(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_compact_all_falsy() {
    assert_eq!(umt_compact(&[0, 0, 0]), Vec::<i32>::new());
}

#[test]
fn test_compact_empty_array() {
    assert_eq!(umt_compact::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn test_compact_floats() {
    let arr = vec![0.0, 1.0, f64::NAN, 2.0, 0.0, 3.0];
    let result = umt_compact(&arr);
    assert_eq!(result, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_compact_booleans() {
    assert_eq!(umt_compact(&[false, true, false, true]), vec![true, true]);
}

#[test]
fn test_compact_strings() {
    assert_eq!(
        umt_compact(&["", "a", "", "b", "c"]),
        vec!["a", "b", "c"]
    );
}

#[test]
fn test_compact_options() {
    assert_eq!(
        umt_compact_options(&[None, Some(1), None, Some(2), Some(3)]),
        vec![1, 2, 3]
    );
}

#[test]
fn test_compact_infinity_is_truthy() {
    let result = umt_compact(&[0.0, f64::INFINITY, f64::NEG_INFINITY]);
    assert_eq!(result, vec![f64::INFINITY, f64::NEG_INFINITY]);
}

#[test]
fn test_compact_does_not_modify_original() {
    let original = vec![0, 1, 0, 2];
    let result = umt_compact(&original);
    assert_eq!(result, vec![1, 2]);
    assert_eq!(original, vec![0, 1, 0, 2]);
}

use umt_rust::array::*;

#[test]
fn test_compact_bools() {
    assert_eq!(umt_compact(&[false, true, false, true]), vec![true, true]);
}

#[test]
fn test_compact_empty() {
    assert_eq!(umt_compact::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn test_compact_integers() {
    assert_eq!(umt_compact(&[0, 1, 0, 2, 0, 3]), vec![1, 2, 3]);
    assert_eq!(umt_compact(&[1, 2, 3]), vec![1, 2, 3]);
    assert_eq!(umt_compact(&[0, 0, 0]), Vec::<i32>::new());
}
