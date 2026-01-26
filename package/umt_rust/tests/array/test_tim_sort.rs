use umt_rust::array::umt_tim_sort;

#[test]
fn test_tim_sort_empty() {
    assert_eq!(umt_tim_sort::<i32>(&[], None, None, None), Vec::<i32>::new());
}

#[test]
fn test_tim_sort_already_sorted() {
    assert_eq!(umt_tim_sort(&[1, 2, 3], None, None, None), vec![1, 2, 3]);
}

#[test]
fn test_tim_sort_reverse() {
    assert_eq!(umt_tim_sort(&[3, 2, 1], None, None, None), vec![1, 2, 3]);
}

#[test]
fn test_tim_sort_random() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted = umt_tim_sort(&array, None, None, None);
    let mut expected = array.clone();
    expected.sort();
    assert_eq!(sorted, expected);
}

#[test]
fn test_tim_sort_duplicates() {
    assert_eq!(
        umt_tim_sort(&[2, 3, 3, 1, 2], None, None, None),
        vec![1, 2, 2, 3, 3]
    );
}

#[test]
fn test_tim_sort_large_array() {
    let large_array: Vec<i32> = (0..10_000).rev().collect();
    let sorted = umt_tim_sort(&large_array, None, None, None);
    assert_eq!(sorted, (0..10_000).collect::<Vec<i32>>());
}

#[test]
fn test_tim_sort_negative_numbers() {
    assert_eq!(
        umt_tim_sort(&[5, -1, 3, 2, 4, -5, 1, -2, 0], None, None, None),
        vec![-5, -2, -1, 0, 1, 2, 3, 4, 5]
    );
}

#[test]
fn test_tim_sort_descending() {
    let descending = |a: &i32, b: &i32| -> i32 {
        if a < b { 1 } else if a > b { -1 } else { 0 }
    };
    assert_eq!(
        umt_tim_sort(&[1, 2, 3], Some(descending), None, None),
        vec![3, 2, 1]
    );
}

#[test]
fn test_tim_sort_partial_range() {
    assert_eq!(
        umt_tim_sort(&[1, 3, 2, 5, 4], None, Some(1), Some(3)),
        vec![1, 2, 3, 5, 4]
    );
}

#[test]
fn test_tim_sort_all_identical() {
    let array = vec![1; 100];
    assert_eq!(umt_tim_sort(&array, None, None, None), array);
}

#[test]
fn test_tim_sort_strings() {
    let strings = vec!["b", "a", "d", "c"];
    assert_eq!(
        umt_tim_sort(&strings, None, None, None),
        vec!["a", "b", "c", "d"]
    );
}

#[test]
fn test_tim_sort_booleans() {
    let array = vec![true, false, true, false];
    assert_eq!(
        umt_tim_sort(&array, None, None, None),
        vec![false, false, true, true]
    );
}

#[test]
fn test_tim_sort_just_larger_than_min_run() {
    let array: Vec<i32> = (0..33).rev().collect();
    let result = umt_tim_sort(&array, None, None, None);
    assert_eq!(result, (0..33).collect::<Vec<i32>>());
}

#[test]
fn test_tim_sort_does_not_mutate_original() {
    let arr = vec![3, 1, 4, 1, 5];
    let _ = umt_tim_sort(&arr, None, None, None);
    assert_eq!(arr, vec![3, 1, 4, 1, 5]);
}

use umt_rust::array::*;

#[test]
fn test_get_min_run_length() {
    assert!(get_min_run_length(100) >= 16);
    assert!(get_min_run_length(100) <= 64);
}

#[test]
fn test_tim_sort_basic() {
    let arr = vec![3, 1, 4, 1, 5];
    assert_eq!(umt_tim_sort(&arr, None, None, None), vec![1, 1, 3, 4, 5]);
}

#[test]
fn test_tim_sort_does_not_mutate() {
    let arr = vec![3, 1, 4, 1, 5];
    let _ = umt_tim_sort(&arr, None, None, None);
    assert_eq!(arr, vec![3, 1, 4, 1, 5]);
}

#[test]
fn test_tim_sort_in_place() {
    let mut arr = vec![3, 1, 4, 1, 5];
    umt_tim_sort_in_place(&mut arr, None);
    assert_eq!(arr, vec![1, 1, 3, 4, 5]);
}

#[test]
fn test_tim_sort_large() {
    let arr: Vec<i32> = (0..1000).rev().collect();
    let sorted = umt_tim_sort(&arr, None, None, None);
    assert_eq!(sorted, (0..1000).collect::<Vec<i32>>());
}

#[test]
fn test_tim_sort_single() {
    let arr = vec![42];
    assert_eq!(umt_tim_sort(&arr, None, None, None), vec![42]);
}

#[test]
fn test_tim_sort_with_custom_compare() {
    let arr = vec![1, 2, 3, 4, 5];
    // Sort in descending order
    let descending = |a: &i32, b: &i32| -> i32 {
        if a < b {
            1
        } else if a > b {
            -1
        } else {
            0
        }
    };
    assert_eq!(
        umt_tim_sort(&arr, Some(descending), None, None),
        vec![5, 4, 3, 2, 1]
    );
}
