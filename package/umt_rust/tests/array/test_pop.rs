use umt_rust::array::umt_pop;

#[test]
fn test_pop_returns_last_element() {
    let array = vec![1, 2, 3];
    let result = umt_pop(&array);
    assert_eq!(result, Some(&3));
    // Verify original array is unchanged
    assert_eq!(array, vec![1, 2, 3]);
}

#[test]
fn test_pop_empty_array() {
    let array: Vec<i32> = vec![];
    let result = umt_pop(&array);
    assert_eq!(result, None);
}

#[test]
fn test_pop_single_element() {
    let array = vec![42];
    let result = umt_pop(&array);
    assert_eq!(result, Some(&42));
    assert_eq!(array, vec![42]);
}

#[test]
fn test_pop_with_strings() {
    let array = vec!["one", "two", "three"];
    let result = umt_pop(&array);
    assert_eq!(result, Some(&"three"));
    assert_eq!(array, vec!["one", "two", "three"]);
}

#[test]
fn test_pop_nested_arrays() {
    let array = vec![vec![1, 2, 3], vec![4, 5]];
    let result = umt_pop(&array);
    assert_eq!(result, Some(&vec![4, 5]));
    assert_eq!(array, vec![vec![1, 2, 3], vec![4, 5]]);
}

#[test]
fn test_pop_does_not_modify_array() {
    let array = vec![1, 2, 3, 4, 5];
    let original = array.clone();
    let _ = umt_pop(&array);
    assert_eq!(array, original);
}
