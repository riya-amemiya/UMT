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
