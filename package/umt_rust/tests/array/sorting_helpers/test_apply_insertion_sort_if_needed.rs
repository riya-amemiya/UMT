use umt_rust::array::sorting_helpers::*;

fn default_compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

#[test]
fn test_apply_insertion_sort_if_needed_small() {
    let mut arr = vec![5, 2, 8, 1, 9];
    let applied = apply_insertion_sort_if_needed(&mut arr, 0, 4, &default_compare, 10);
    assert!(applied);
    assert_eq!(arr, vec![1, 2, 5, 8, 9]);
}

#[test]
fn test_apply_insertion_sort_if_needed_large() {
    let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6, 0, 10];
    let original = arr.clone();
    let applied = apply_insertion_sort_if_needed(&mut arr, 0, 10, &default_compare, 5);
    assert!(!applied);
    assert_eq!(arr, original);
}

#[test]
fn test_apply_insertion_sort_if_needed_exact_threshold() {
    let mut arr = vec![5, 2, 8, 1, 9];
    let applied = apply_insertion_sort_if_needed(&mut arr, 0, 4, &default_compare, 5);
    assert!(applied);
    assert_eq!(arr, vec![1, 2, 5, 8, 9]);
}

#[test]
fn test_apply_insertion_sort_if_needed_partial_range() {
    let mut arr = vec![5, 2, 8, 1, 9];
    let applied = apply_insertion_sort_if_needed(&mut arr, 1, 3, &default_compare, 5);
    assert!(applied);
    assert_eq!(arr, vec![5, 1, 2, 8, 9]);
}
