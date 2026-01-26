use std::collections::HashSet;
use umt_rust::array::{umt_shuffle, umt_shuffle_in_place};

#[test]
fn test_shuffle_maintains_length() {
    let array = vec![1, 2, 3, 4, 5];
    let shuffled = umt_shuffle(&array);
    assert_eq!(shuffled.len(), array.len());
}

#[test]
fn test_shuffle_maintains_elements() {
    let array = vec![1, 2, 3, 4, 5];
    let shuffled = umt_shuffle(&array);

    let original_set: HashSet<_> = array.iter().collect();
    let shuffled_set: HashSet<_> = shuffled.iter().collect();
    assert_eq!(original_set, shuffled_set);
}

#[test]
fn test_shuffle_empty_array() {
    let array: Vec<i32> = vec![];
    let shuffled = umt_shuffle(&array);
    assert_eq!(shuffled, Vec::<i32>::new());
}

#[test]
fn test_shuffle_single_element() {
    let array = vec![1];
    let shuffled = umt_shuffle(&array);
    assert_eq!(shuffled, vec![1]);
}

#[test]
fn test_shuffle_does_not_modify_original() {
    let array = vec![1, 2, 3, 4, 5];
    let original = array.clone();
    let _ = umt_shuffle(&array);
    assert_eq!(array, original);
}

#[test]
fn test_shuffle_produces_different_orders() {
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut same_count = 0;

    for _ in 0..10 {
        let shuffled = umt_shuffle(&array);
        if shuffled == array {
            same_count += 1;
        }
    }

    // It's very unlikely to get the same order 10 times
    assert!(same_count < 10);
}

#[test]
fn test_shuffle_with_duplicates() {
    let array = vec![1, 1, 2, 2, 3, 3];
    let shuffled = umt_shuffle(&array);

    // Count elements
    let count = |arr: &[i32], val: i32| arr.iter().filter(|&&x| x == val).count();

    assert_eq!(count(&shuffled, 1), 2);
    assert_eq!(count(&shuffled, 2), 2);
    assert_eq!(count(&shuffled, 3), 2);
}

#[test]
fn test_shuffle_strings() {
    let array = vec!["a", "b", "c", "d", "e"];
    let shuffled = umt_shuffle(&array);

    let original_set: HashSet<_> = array.iter().collect();
    let shuffled_set: HashSet<_> = shuffled.iter().collect();
    assert_eq!(original_set, shuffled_set);
}

#[test]
fn test_shuffle_in_place() {
    let mut array = vec![1, 2, 3, 4, 5];
    let original_set: HashSet<_> = array.iter().cloned().collect();

    umt_shuffle_in_place(&mut array);

    let shuffled_set: HashSet<_> = array.iter().cloned().collect();
    assert_eq!(original_set, shuffled_set);
}

#[test]
fn test_shuffle_large_array() {
    let array: Vec<i32> = (0..10_000).collect();
    let shuffled = umt_shuffle(&array);

    assert_eq!(shuffled.len(), array.len());

    let original_set: HashSet<_> = array.iter().collect();
    let shuffled_set: HashSet<_> = shuffled.iter().collect();
    assert_eq!(original_set, shuffled_set);
}
