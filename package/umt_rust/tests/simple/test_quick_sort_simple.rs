//! Quick Sort Simple integration tests.
//!
//! Ported from TypeScript: quickSortSimple.test.ts

use std::cmp::Ordering;
use umt_rust::simple::array::{
    compare_function_default, umt_quick_sort_simple, umt_quick_sort_simple_f64,
    umt_quick_sort_simple_i32,
};

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

#[test]
fn test_f64_sort_ascending() {
    let arr = vec![3.0, 1.0, 4.0, 1.0, 5.0];
    let result = umt_quick_sort_simple_f64(&arr, true, None, None);
    assert_eq!(result, vec![1.0, 1.0, 3.0, 4.0, 5.0]);
}

#[test]
fn test_f64_sort_descending() {
    let arr = vec![3.0, 1.0, 4.0, 1.0, 5.0];
    let result = umt_quick_sort_simple_f64(&arr, false, None, None);
    assert_eq!(result, vec![5.0, 4.0, 3.0, 1.0, 1.0]);
}

#[test]
fn test_f64_empty_array() {
    let arr: Vec<f64> = vec![];
    let result = umt_quick_sort_simple_f64(&arr, true, None, None);
    assert_eq!(result, Vec::<f64>::new());
}

#[test]
fn test_f64_with_range() {
    let arr = vec![5.0, 3.0, 8.0, 1.0, 9.0];
    let result = umt_quick_sort_simple_f64(&arr, true, Some(1), Some(3));
    // Should sort indices 1-3: [3.0, 8.0, 1.0] -> [1.0, 3.0, 8.0]
    assert_eq!(result, vec![5.0, 1.0, 3.0, 8.0, 9.0]);
}

#[test]
fn test_compare_function_default() {
    assert_eq!(compare_function_default(&1, &2), Ordering::Less);
    assert_eq!(compare_function_default(&2, &1), Ordering::Greater);
    assert_eq!(compare_function_default(&1, &1), Ordering::Equal);
}

#[test]
fn test_handles_end_id_negative() {
    let arr = vec![3, 1, 4];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(0), Some(-1));
    assert_eq!(result, vec![1, 3, 4]);
}

#[test]
fn test_sort_large_array() {
    // Large array to trigger insertion sort threshold
    let arr: Vec<i32> = (0..100).rev().collect();
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    let expected: Vec<i32> = (0..100).collect();
    assert_eq!(result, expected);
}

#[test]
fn test_sort_with_custom_range() {
    let arr = vec![5, 3, 8, 1, 9, 2, 7];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(2), Some(5));
    // Should sort indices 2-5: [8, 1, 9, 2] -> [1, 2, 8, 9]
    assert_eq!(result, vec![5, 3, 1, 2, 8, 9, 7]);
}

#[test]
fn test_sort_two_elements() {
    let arr = vec![2, 1];
    let result = umt_quick_sort_simple_i32(&arr, true, None, None);
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_sort_three_elements_all_permutations() {
    // Test all permutations of 3 elements to exercise median-of-three
    let permutations = vec![
        vec![1, 2, 3],
        vec![1, 3, 2],
        vec![2, 1, 3],
        vec![2, 3, 1],
        vec![3, 1, 2],
        vec![3, 2, 1],
    ];
    for perm in permutations {
        let result = umt_quick_sort_simple_i32(&perm, true, None, None);
        assert_eq!(result, vec![1, 2, 3]);
    }
}

#[test]
fn test_sort_with_equal_start_and_end() {
    let arr = vec![3, 1, 4];
    let result = umt_quick_sort_simple_i32(&arr, true, Some(1), Some(1));
    // Single element range, no sorting needed
    assert_eq!(result, vec![3, 1, 4]);
}
