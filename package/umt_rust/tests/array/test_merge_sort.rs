use umt_rust::array::umt_merge_sort;

#[test]
fn test_merge_sort_empty() {
    assert_eq!(
        umt_merge_sort::<i32>(&[], None),
        Vec::<i32>::new()
    );
}

#[test]
fn test_merge_sort_already_sorted() {
    assert_eq!(umt_merge_sort(&[1, 2, 3], None), vec![1, 2, 3]);
}

#[test]
fn test_merge_sort_reverse() {
    assert_eq!(umt_merge_sort(&[3, 2, 1], None), vec![1, 2, 3]);
}

#[test]
fn test_merge_sort_random() {
    assert_eq!(umt_merge_sort(&[2, 3, 1], None), vec![1, 2, 3]);
}

#[test]
fn test_merge_sort_duplicates() {
    assert_eq!(
        umt_merge_sort(&[2, 3, 3, 1, 2], None),
        vec![1, 2, 2, 3, 3]
    );
}

#[test]
fn test_merge_sort_single_element() {
    assert_eq!(umt_merge_sort(&[1], None), vec![1]);
}

#[test]
fn test_merge_sort_negative_numbers() {
    assert_eq!(
        umt_merge_sort(&[3, -1, 4, -5, 2, -3], None),
        vec![-5, -3, -1, 2, 3, 4]
    );
}

#[test]
fn test_merge_sort_strings() {
    assert_eq!(
        umt_merge_sort(&["banana", "apple", "cherry"], None),
        vec!["apple", "banana", "cherry"]
    );
}

#[test]
fn test_merge_sort_descending() {
    let descending = |a: &i32, b: &i32| -> i32 {
        if a < b { 1 } else if a > b { -1 } else { 0 }
    };
    assert_eq!(umt_merge_sort(&[1, 3, 2], Some(descending)), vec![3, 2, 1]);
}

#[test]
fn test_merge_sort_by_length() {
    let by_length = |a: &&str, b: &&str| -> i32 {
        let diff = a.len() as i32 - b.len() as i32;
        if diff > 0 { 1 } else if diff < 0 { -1 } else { 0 }
    };
    assert_eq!(
        umt_merge_sort(&["aaa", "a", "aa"], Some(by_length)),
        vec!["a", "aa", "aaa"]
    );
}

#[test]
fn test_merge_sort_large_array() {
    let arr: Vec<i32> = (0..1000).rev().collect();
    let sorted = umt_merge_sort(&arr, None);
    assert_eq!(sorted, (0..1000).collect::<Vec<i32>>());
}

#[test]
fn test_merge_sort_does_not_mutate_original() {
    let arr = vec![3, 1, 4, 1, 5];
    let _ = umt_merge_sort(&arr, None);
    assert_eq!(arr, vec![3, 1, 4, 1, 5]);
}

use umt_rust::array::*;

#[test]
fn test_merge_sort_basic() {
    let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
    assert_eq!(umt_merge_sort(&arr, None), vec![1, 1, 2, 3, 4, 5, 6, 9]);
}

#[test]
fn test_merge_sort_does_not_mutate() {
    let arr = vec![3, 1, 4, 1, 5];
    let _ = umt_merge_sort(&arr, None);
    assert_eq!(arr, vec![3, 1, 4, 1, 5]);
}

#[test]
fn test_merge_sort_floats() {
    let arr = vec![3.14, 2.71, 1.41, 1.73];
    let sorted = umt_merge_sort(&arr, None);
    assert_eq!(sorted, vec![1.41, 1.73, 2.71, 3.14]);
}

#[test]
fn test_merge_sort_large() {
    let arr: Vec<i32> = (0..1000).rev().collect();
    let sorted = umt_merge_sort(&arr, None);
    assert_eq!(sorted, (0..1000).collect::<Vec<i32>>());
}

#[test]
fn test_merge_sort_single() {
    let arr = vec![42];
    assert_eq!(umt_merge_sort(&arr, None), vec![42]);
}

#[test]
fn test_merge_sort_with_custom_compare() {
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
    assert_eq!(umt_merge_sort(&arr, Some(descending)), vec![5, 4, 3, 2, 1]);
}
