use umt_rust::array::umt_dual_pivot_quick_sort;

#[test]
fn test_dual_pivot_quick_sort_numbers() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, None),
        vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
    );
}

#[test]
fn test_dual_pivot_quick_sort_strings() {
    let array = vec!["banana", "apple", "orange", "grape"];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, None),
        vec!["apple", "banana", "grape", "orange"]
    );
}

#[test]
fn test_dual_pivot_quick_sort_descending() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let compare_fn = |a: &i32, b: &i32| -> i32 {
        if a < b {
            1
        } else if a > b {
            -1
        } else {
            0
        }
    };
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, Some(compare_fn), None, None, None),
        vec![9, 6, 5, 5, 4, 3, 3, 2, 1, 1]
    );
}

#[test]
fn test_dual_pivot_quick_sort_empty() {
    assert_eq!(
        umt_dual_pivot_quick_sort::<i32>(&[], None, None, None, None),
        Vec::<i32>::new()
    );
}

#[test]
fn test_dual_pivot_quick_sort_single_element() {
    assert_eq!(
        umt_dual_pivot_quick_sort(&[1], None, None, None, None),
        vec![1]
    );
}

#[test]
fn test_dual_pivot_quick_sort_all_identical() {
    let array = vec![2, 2, 2, 2, 2];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, None),
        vec![2, 2, 2, 2, 2]
    );
}

#[test]
fn test_dual_pivot_quick_sort_already_sorted() {
    let array = vec![1, 2, 3, 4, 5];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, None),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_dual_pivot_quick_sort_reverse_sorted() {
    let array = vec![5, 4, 3, 2, 1];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, None),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_dual_pivot_quick_sort_negative_numbers() {
    let array = vec![-3, 1, -4, 1, 5, -9, 2, 6, 5, 3];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, None),
        vec![-9, -4, -3, 1, 1, 2, 3, 5, 5, 6]
    );
}

#[test]
fn test_dual_pivot_quick_sort_partial_range() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, Some(2), Some(6), None),
        vec![3, 1, 1, 2, 4, 5, 9, 6, 5, 3]
    );
}

#[test]
fn test_dual_pivot_quick_sort_large_array() {
    let array: Vec<i32> = (0..1000).rev().collect();
    let sorted = umt_dual_pivot_quick_sort(&array, None, None, None, None);
    assert_eq!(sorted, (0..1000).collect::<Vec<i32>>());
}

#[test]
fn test_dual_pivot_quick_sort_custom_threshold() {
    let array = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    assert_eq!(
        umt_dual_pivot_quick_sort(&array, None, None, None, Some(5)),
        vec![1, 1, 2, 3, 3, 4, 5, 5, 6, 9]
    );
}
