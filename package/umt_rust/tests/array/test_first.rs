use umt_rust::array::umt_first;

#[test]
fn test_first_returns_first_element() {
    assert_eq!(umt_first(&[1, 2, 3]), Some(&1));
    assert_eq!(umt_first(&["a", "b", "c"]), Some(&"a"));
    assert_eq!(umt_first(&[true, false, true]), Some(&true));
}

#[test]
fn test_first_empty_array() {
    assert_eq!(umt_first::<i32>(&[]), None);
}

#[test]
fn test_first_single_element() {
    assert_eq!(umt_first(&[42]), Some(&42));
    assert_eq!(umt_first(&["single"]), Some(&"single"));
}

#[test]
fn test_first_with_special_numeric_values() {
    assert!(umt_first(&[f64::NAN, 1.0, 2.0]).unwrap().is_nan());
    assert_eq!(umt_first(&[f64::INFINITY, 1.0, 2.0]), Some(&f64::INFINITY));
    assert_eq!(umt_first(&[f64::NEG_INFINITY, 1.0, 2.0]), Some(&f64::NEG_INFINITY));
    assert_eq!(umt_first(&[0.0, 1.0, 2.0]), Some(&0.0));
}

#[test]
fn test_first_preserves_reference() {
    let arr = vec![1, 2, 3];
    let first = umt_first(&arr);
    assert_eq!(first, Some(&1));
    // Verify we get a reference to the original
    assert!(std::ptr::eq(first.unwrap(), &arr[0]));
}

#[test]
fn test_first_nested_arrays() {
    let nested_array1 = vec![1, 2, 3];
    let nested_array2 = vec![4, 5, 6];
    let arr = vec![nested_array1.clone(), nested_array2];
    assert_eq!(umt_first(&arr), Some(&nested_array1));
}

#[test]
fn test_first_large_array() {
    let large_array: Vec<i32> = (0..10_000).collect();
    assert_eq!(umt_first(&large_array), Some(&0));

    let large_string_array: Vec<String> = (0..10_000).map(|i| format!("item{}", i)).collect();
    assert_eq!(umt_first(&large_string_array), Some(&"item0".to_string()));
}
