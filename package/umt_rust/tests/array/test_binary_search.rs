use umt_rust::array::{umt_binary_search, umt_binary_search_i32};

#[test]
fn test_binary_search_found() {
    let array = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    assert_eq!(umt_binary_search(&array, 7), Some(3));
    assert_eq!(umt_binary_search(&array, 1), Some(0));
    assert_eq!(umt_binary_search(&array, 19), Some(9));
}

#[test]
fn test_binary_search_not_found() {
    let array = vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19];
    assert_eq!(umt_binary_search(&array, 2), None);
    assert_eq!(umt_binary_search(&array, 20), None);
    assert_eq!(umt_binary_search(&array, 0), None);
}

#[test]
fn test_binary_search_empty_array() {
    let array: Vec<i32> = vec![];
    assert_eq!(umt_binary_search(&array, 1), None);
}

#[test]
fn test_binary_search_single_element() {
    let array = vec![5];
    assert_eq!(umt_binary_search(&array, 5), Some(0));
    assert_eq!(umt_binary_search(&array, 1), None);
}

#[test]
fn test_binary_search_two_elements() {
    let array = vec![3, 6];
    assert_eq!(umt_binary_search(&array, 3), Some(0));
    assert_eq!(umt_binary_search(&array, 6), Some(1));
    assert_eq!(umt_binary_search(&array, 4), None);
}

#[test]
fn test_binary_search_i32() {
    let array = vec![1, 3, 5, 7, 9];
    assert_eq!(umt_binary_search_i32(&array, 5), 2);
    assert_eq!(umt_binary_search_i32(&array, 4), -1);
}

#[test]
fn test_binary_search_strings() {
    let array = vec!["apple", "banana", "cherry", "date"];
    assert_eq!(umt_binary_search(&array, "banana"), Some(1));
    assert_eq!(umt_binary_search(&array, "fig"), None);
}

#[test]
fn test_binary_search_floats() {
    let array = vec![1.0, 2.5, 3.7, 5.2, 8.9];
    assert_eq!(umt_binary_search(&array, 3.7), Some(2));
    assert_eq!(umt_binary_search(&array, 4.0), None);
}

#[test]
fn test_binary_search_first_element() {
    let array = vec![1, 2, 3, 4, 5];
    assert_eq!(umt_binary_search(&array, 1), Some(0));
}

#[test]
fn test_binary_search_last_element() {
    let array = vec![1, 2, 3, 4, 5];
    assert_eq!(umt_binary_search(&array, 5), Some(4));
}

#[test]
fn test_binary_search_middle_element() {
    let array = vec![1, 2, 3, 4, 5];
    assert_eq!(umt_binary_search(&array, 3), Some(2));
}
