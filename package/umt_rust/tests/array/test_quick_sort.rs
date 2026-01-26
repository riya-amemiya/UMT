use umt_rust::array::{umt_quick_sort, umt_quick_sort_in_place};

#[test]
fn test_quick_sort_empty() {
    assert_eq!(
        umt_quick_sort::<i32>(&[], None, None, None, None),
        Vec::<i32>::new()
    );
}

#[test]
fn test_quick_sort_already_sorted() {
    assert_eq!(
        umt_quick_sort(&[1, 2, 3, 4, 5], None, None, None, None),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_quick_sort_reverse() {
    assert_eq!(
        umt_quick_sort(&[5, 4, 3, 2, 1], None, None, None, None),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_quick_sort_random() {
    assert_eq!(
        umt_quick_sort(&[3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5], None, None, None, None),
        vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]
    );
}

#[test]
fn test_quick_sort_duplicates() {
    assert_eq!(
        umt_quick_sort(&[3, 3, 3, 2, 1, 1, 4, 4, 5], None, None, None, None),
        vec![1, 1, 2, 3, 3, 3, 4, 4, 5]
    );
}

#[test]
fn test_quick_sort_large_array() {
    let large_array: Vec<i32> = (0..10_000).rev().collect();
    let sorted = umt_quick_sort(&large_array, None, None, None, None);
    assert_eq!(sorted, (0..10_000).collect::<Vec<i32>>());
}

#[test]
fn test_quick_sort_negative_numbers() {
    assert_eq!(
        umt_quick_sort(&[5, -1, 3, 2, 4, -5, 1, -2, 0], None, None, None, None),
        vec![-5, -2, -1, 0, 1, 2, 3, 4, 5]
    );
}

#[test]
fn test_quick_sort_descending() {
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
        umt_quick_sort(&[1, 2, 3, 4, 5], Some(descending), None, None, None),
        vec![5, 4, 3, 2, 1]
    );
}

#[test]
fn test_quick_sort_partial_range() {
    assert_eq!(
        umt_quick_sort(&[3, 1, 4, 1, 5], None, Some(1), Some(3), None),
        vec![3, 1, 1, 4, 5]
    );
}

#[test]
fn test_quick_sort_single_element() {
    assert_eq!(umt_quick_sort(&[1], None, None, None, None), vec![1]);
}

#[test]
fn test_quick_sort_median_of_three_all_equal() {
    assert_eq!(
        umt_quick_sort(&[5, 5, 5, 5, 5], None, None, None, None),
        vec![5, 5, 5, 5, 5]
    );
}

#[test]
fn test_quick_sort_three_element_arrays() {
    assert_eq!(
        umt_quick_sort(&[1, 2, 3], None, None, None, None),
        vec![1, 2, 3]
    );
    assert_eq!(
        umt_quick_sort(&[1, 3, 2], None, None, None, None),
        vec![1, 2, 3]
    );
    assert_eq!(
        umt_quick_sort(&[2, 1, 3], None, None, None, None),
        vec![1, 2, 3]
    );
    assert_eq!(
        umt_quick_sort(&[2, 3, 1], None, None, None, None),
        vec![1, 2, 3]
    );
    assert_eq!(
        umt_quick_sort(&[3, 1, 2], None, None, None, None),
        vec![1, 2, 3]
    );
    assert_eq!(
        umt_quick_sort(&[3, 2, 1], None, None, None, None),
        vec![1, 2, 3]
    );
}

#[test]
fn test_quick_sort_threshold_cases() {
    let arr: Vec<i32> = (0..10).rev().collect();
    assert_eq!(
        umt_quick_sort(&arr, None, None, None, None),
        (0..10).collect::<Vec<i32>>()
    );

    let arr: Vec<i32> = (0..5).rev().collect();
    assert_eq!(
        umt_quick_sort(&arr, None, None, None, None),
        (0..5).collect::<Vec<i32>>()
    );
}

#[test]
fn test_quick_sort_custom_threshold() {
    let arr: Vec<i32> = (0..20).rev().collect();
    assert_eq!(
        umt_quick_sort(&arr, None, None, None, Some(5)),
        (0..20).collect::<Vec<i32>>()
    );
}

#[test]
fn test_quick_sort_boundary_conditions() {
    assert_eq!(umt_quick_sort(&[2, 1], None, None, None, None), vec![1, 2]);
    assert_eq!(umt_quick_sort(&[1, 2], None, None, None, None), vec![1, 2]);
}

#[test]
fn test_quick_sort_all_same_except_one() {
    assert_eq!(
        umt_quick_sort(&[1, 1, 1, 0, 1], None, None, None, None),
        vec![0, 1, 1, 1, 1]
    );
    assert_eq!(
        umt_quick_sort(&[1, 1, 1, 2, 1], None, None, None, None),
        vec![1, 1, 1, 1, 2]
    );
}

#[test]
fn test_quick_sort_in_place() {
    let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
    umt_quick_sort_in_place(&mut arr, None);
    assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
}

#[test]
fn test_quick_sort_does_not_mutate_original() {
    let arr = vec![3, 1, 4, 1, 5];
    let _ = umt_quick_sort(&arr, None, None, None, None);
    assert_eq!(arr, vec![3, 1, 4, 1, 5]);
}
