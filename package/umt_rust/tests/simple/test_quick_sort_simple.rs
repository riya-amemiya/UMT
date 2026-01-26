//! Quick Sort Simple integration tests.
//!
//! Ported from TypeScript: quickSortSimple.test.ts

use std::cmp::Ordering;
use umt_rust::simple::array::{umt_quick_sort_simple, umt_quick_sort_simple_i32};

#[test]
fn test_returns_empty_array_when_sorting_empty_array() {
    let arr: Vec<i32> = vec![];
    let result = umt_quick_sort_simple::<i32, fn(&i32, &i32) -> Ordering>(&arr, None, None, None);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_handles_start_id_outside_array_bounds_negative() {
    let arr = vec![3, 1, 4];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(-1), Some(2));
    assert_eq!(result, vec![1, 3, 4]);
}

#[test]
fn test_handles_start_id_outside_array_bounds_too_large() {
    let arr = vec![3, 1, 4];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(4), Some(2));
    assert_eq!(result, vec![1, 3, 4]);
}

#[test]
fn test_handles_end_id_outside_array_bounds() {
    let arr = vec![3, 1, 4];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(0), Some(5));
    assert_eq!(result, vec![1, 3, 4]);
}

#[test]
fn test_handles_start_id_greater_than_end_id() {
    let arr = vec![3, 1, 4];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(2), Some(1));
    assert_eq!(result, vec![1, 3, 4]);
}

#[test]
fn test_sort_ascending() {
    let arr = vec![5, 2, 8, 1, 9];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![1, 2, 5, 8, 9]);
}

#[test]
fn test_sort_descending() {
    let arr = vec![5, 2, 8, 1, 9];
    let result = umt_quick_sort_simple_i32(&arr, false, None, None);
    assert_eq!(result, vec![9, 8, 5, 2, 1]);
}

#[test]
fn test_sort_with_duplicates() {
    let arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
}

#[test]
fn test_sort_single_element() {
    let arr = vec![42];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![42]);
}

#[test]
fn test_sort_already_sorted() {
    let arr = vec![1, 2, 3, 4, 5];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_sort_reverse_sorted() {
    let arr = vec![5, 4, 3, 2, 1];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

use umt_rust::simple::array::*;

#[test]
fn test_sort_f64_array() {
    let arr = vec![3.14, 1.41, 2.72, 1.61];
    let result = umt_quick_sort_simple_f64(&arr, true, None, None);
    assert_eq!(result, vec![1.41, 1.61, 2.72, 3.14]);
}

#[test]
fn test_sort_string_array() {
    let arr = vec!["banana", "apple", "cherry"];
    let result =
        umt_quick_sort_simple::<&str, fn(&&str, &&str) -> Ordering>(&arr, None, None, None);
    assert_eq!(result, vec!["apple", "banana", "cherry"]);
}

#[test]
fn test_sort_two_elements() {
    let arr = vec![2, 1];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_sort_with_custom_compare_function() {
    let arr = vec![1, 3, 2, 4, 5];
    let result = umt_quick_sort_simple(&arr, Some(|a: &i32, b: &i32| a.cmp(b)), None, None);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_sort_with_partial_range() {
    let arr = vec![5, 4, 3, 2, 1];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(1), Some(3));
    assert_eq!(result, vec![5, 2, 3, 4, 1]);
}
