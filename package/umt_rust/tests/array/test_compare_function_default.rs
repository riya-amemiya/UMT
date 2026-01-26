use umt_rust::array::umt_compare_function_default;

#[test]
fn test_compare_greater() {
    assert_eq!(umt_compare_function_default(&2, &1), 1);
    assert_eq!(umt_compare_function_default(&"b", &"a"), 1);
}

#[test]
fn test_compare_lesser() {
    assert_eq!(umt_compare_function_default(&1, &2), -1);
    assert_eq!(umt_compare_function_default(&"a", &"b"), -1);
}

#[test]
fn test_compare_equal() {
    assert_eq!(umt_compare_function_default(&1, &1), 0);
    assert_eq!(umt_compare_function_default(&"a", &"a"), 0);
}

#[test]
fn test_compare_booleans() {
    assert_eq!(umt_compare_function_default(&true, &false), 1);
    assert_eq!(umt_compare_function_default(&false, &true), -1);
    assert_eq!(umt_compare_function_default(&true, &true), 0);
}

#[test]
fn test_compare_floats() {
    assert_eq!(umt_compare_function_default(&2.0, &1.0), 1);
    assert_eq!(umt_compare_function_default(&1.0, &2.0), -1);
    assert_eq!(umt_compare_function_default(&1.0, &1.0), 0);
}

#[test]
fn test_compare_negative_numbers() {
    assert_eq!(umt_compare_function_default(&-1, &-2), 1);
    assert_eq!(umt_compare_function_default(&-2, &-1), -1);
    assert_eq!(umt_compare_function_default(&-1, &-1), 0);
}

#[test]
fn test_compare_mixed_signs() {
    assert_eq!(umt_compare_function_default(&1, &-1), 1);
    assert_eq!(umt_compare_function_default(&-1, &1), -1);
}

use umt_rust::array::*;

#[test]
fn test_compare_integers() {
    assert_eq!(umt_compare_function_default(&2, &1), 1);
    assert_eq!(umt_compare_function_default(&1, &2), -1);
    assert_eq!(umt_compare_function_default(&2, &2), 0);
}

#[test]
fn test_compare_strings() {
    assert_eq!(umt_compare_function_default(&"b", &"a"), 1);
    assert_eq!(umt_compare_function_default(&"a", &"b"), -1);
    assert_eq!(umt_compare_function_default(&"a", &"a"), 0);
}

#[test]
fn test_compare_zero() {
    assert_eq!(umt_compare_function_default(&0, &0), 0);
    assert_eq!(umt_compare_function_default(&1, &0), 1);
    assert_eq!(umt_compare_function_default(&0, &1), -1);
}
