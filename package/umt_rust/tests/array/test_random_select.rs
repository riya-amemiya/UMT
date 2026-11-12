use std::collections::HashSet;
use umt_rust::array::{umt_random_select, umt_random_select_one};

#[test]
fn test_random_select_count() {
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result = umt_random_select(&array, 5, false);
    assert_eq!(result.len(), 5);
}

#[test]
fn test_random_select_no_duplicates() {
    let array = vec![1, 2, 3, 4, 5];
    let result = umt_random_select(&array, 5, false);

    let unique: HashSet<_> = result.iter().collect();
    assert_eq!(unique.len(), 5);
}

#[test]
fn test_random_select_elements_from_source() {
    let array = vec![1, 2, 3, 4, 5];
    let result = umt_random_select(&array, 3, false);

    for item in &result {
        assert!(array.contains(item));
    }
}

#[test]
fn test_random_select_more_than_array_no_duplicates() {
    let array = vec![1, 2, 3];
    let result = umt_random_select(&array, 10, false);

    // Without duplicates, can only select up to array length
    assert_eq!(result.len(), 3);
}

#[test]
fn test_random_select_with_duplicates() {
    let array = vec![1, 2, 3];
    let result = umt_random_select(&array, 10, true);

    // With duplicates, can select the requested count
    assert_eq!(result.len(), 10);
}

#[test]
fn test_random_select_empty_array() {
    let array: Vec<i32> = vec![];
    let result = umt_random_select(&array, 5, false);
    assert!(result.is_empty());
}

#[test]
fn test_random_select_zero_count() {
    let array = vec![1, 2, 3, 4, 5];
    let result = umt_random_select(&array, 0, false);
    assert!(result.is_empty());
}

#[test]
fn test_random_select_one() {
    let array = vec![1, 2, 3, 4, 5];
    let result = umt_random_select_one(&array);

    assert!(result.is_some());
    assert!(array.contains(&result.unwrap()));
}

#[test]
fn test_random_select_one_empty() {
    let array: Vec<i32> = vec![];
    let result = umt_random_select_one(&array);
    assert!(result.is_none());
}

#[test]
fn test_random_select_strings() {
    let array = vec!["a", "b", "c", "d", "e"];
    let result = umt_random_select(&array, 3, false);

    assert_eq!(result.len(), 3);
    for item in &result {
        assert!(array.contains(item));
    }
}

#[test]
fn test_random_select_single_element_array() {
    let array = vec![42];
    let result = umt_random_select(&array, 1, false);
    assert_eq!(result, vec![42]);
}

#[test]
fn test_random_select_produces_random_results() {
    let array: Vec<i32> = (0..100).collect();
    let result1 = umt_random_select(&array, 10, false);
    let result2 = umt_random_select(&array, 10, false);

    // It's very unlikely to get the same result twice
    // (though not impossible, so we just check they're valid)
    assert_eq!(result1.len(), 10);
    assert_eq!(result2.len(), 10);
}

#[test]
fn test_random_select_most_of_large_array_is_unique() {
    let array: Vec<i32> = (0..10_000).collect();
    let result = umt_random_select(&array, 9_000, false);
    assert_eq!(result.len(), 9_000);
    let unique: HashSet<i32> = result.iter().copied().collect();
    assert_eq!(unique.len(), 9_000);
    assert!(result.iter().all(|value| (0..10_000).contains(value)));
}

#[test]
fn test_random_select_whole_array_is_permutation() {
    let array: Vec<i32> = (0..500).collect();
    let mut result = umt_random_select(&array, 500, false);
    result.sort_unstable();
    assert_eq!(result, array);

    let mut over = umt_random_select(&array, 5_000, false);
    over.sort_unstable();
    assert_eq!(over, array);
}

#[test]
fn test_random_select_below_shuffle_threshold_is_unique() {
    let array: Vec<i32> = (0..10).collect();
    let result = umt_random_select(&array, 8, false);
    assert_eq!(result.len(), 8);
    let unique: HashSet<i32> = result.iter().copied().collect();
    assert_eq!(unique.len(), 8);
}
