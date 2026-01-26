use umt_rust::array::{umt_insertion_sort, umt_insertion_sort_in_place};

#[test]
fn test_insertion_sort_empty() {
    assert_eq!(
        umt_insertion_sort::<i32>(&[], None, None, None),
        Vec::<i32>::new()
    );
}

#[test]
fn test_insertion_sort_already_sorted() {
    assert_eq!(
        umt_insertion_sort(&[1, 2, 3], None, None, None),
        vec![1, 2, 3]
    );
}

#[test]
fn test_insertion_sort_reverse() {
    assert_eq!(
        umt_insertion_sort(&[3, 2, 1], None, None, None),
        vec![1, 2, 3]
    );
}

#[test]
fn test_insertion_sort_random() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let sorted = umt_insertion_sort(&array, None, None, None);
    let mut expected = array.clone();
    expected.sort();
    assert_eq!(sorted, expected);
}

#[test]
fn test_insertion_sort_duplicates() {
    assert_eq!(
        umt_insertion_sort(&[2, 3, 3, 1, 2], None, None, None),
        vec![1, 2, 2, 3, 3]
    );
}

#[test]
fn test_insertion_sort_negative_numbers() {
    assert_eq!(
        umt_insertion_sort(&[5, -1, 3, 2, 4, -5, 1, -2, 0], None, None, None),
        vec![-5, -2, -1, 0, 1, 2, 3, 4, 5]
    );
}

#[test]
fn test_insertion_sort_descending() {
    let descending = |a: &i32, b: &i32| -> i32 {
        if a < b { 1 } else if a > b { -1 } else { 0 }
    };
    assert_eq!(
        umt_insertion_sort(&[1, 2, 3], Some(descending), None, None),
        vec![3, 2, 1]
    );
}

#[test]
fn test_insertion_sort_partial_range() {
    assert_eq!(
        umt_insertion_sort(&[1, 3, 2, 5, 4], None, Some(1), Some(3)),
        vec![1, 2, 3, 5, 4]
    );
}

#[test]
fn test_insertion_sort_strings() {
    assert_eq!(
        umt_insertion_sort(&["banana", "apple", "cherry"], None, None, None),
        vec!["apple", "banana", "cherry"]
    );
}

#[test]
fn test_insertion_sort_single_element() {
    assert_eq!(
        umt_insertion_sort(&[42], None, None, None),
        vec![42]
    );
}

#[test]
fn test_insertion_sort_in_place() {
    let mut arr = vec![3, 1, 4, 1, 5];
    umt_insertion_sort_in_place(&mut arr, None, None, None);
    assert_eq!(arr, vec![1, 1, 3, 4, 5]);
}

#[test]
fn test_insertion_sort_does_not_mutate_original() {
    let arr = vec![3, 1, 4, 1, 5];
    let _ = umt_insertion_sort(&arr, None, None, None);
    assert_eq!(arr, vec![3, 1, 4, 1, 5]);
}

use umt_rust::array::*;

#[test]
fn test_insertion_sort_basic() {
    let arr = vec![4, 2, 7, 1, 3];
    assert_eq!(
        umt_insertion_sort(&arr, None, None, None),
        vec![1, 2, 3, 4, 7]
    );
}

#[test]
fn test_insertion_sort_does_not_mutate() {
    let arr = vec![4, 2, 7, 1, 3];
    let _ = umt_insertion_sort(&arr, None, None, None);
    assert_eq!(arr, vec![4, 2, 7, 1, 3]);
}

#[test]
fn test_insertion_sort_single() {
    let arr = vec![42];
    assert_eq!(umt_insertion_sort(&arr, None, None, None), vec![42]);
}

#[test]
fn test_insertion_sort_with_custom_compare() {
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
        umt_insertion_sort(&arr, Some(descending), None, None),
        vec![5, 4, 3, 2, 1]
    );
}
