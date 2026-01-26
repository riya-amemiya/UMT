use umt_rust::array::{umt_unique, umt_unique_f64};

#[test]
fn test_unique_removes_duplicates() {
    assert_eq!(umt_unique(&[1, 2, 2, 3, 3, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_unique_empty_array() {
    assert_eq!(umt_unique::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn test_unique_no_duplicates() {
    assert_eq!(umt_unique(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn test_unique_strings() {
    assert_eq!(umt_unique(&["a", "b", "b", "c", "a"]), vec!["a", "b", "c"]);
}

#[test]
fn test_unique_all_same() {
    assert_eq!(umt_unique(&[1, 1, 1, 1]), vec![1]);
}

#[test]
fn test_unique_preserves_order() {
    assert_eq!(umt_unique(&[3, 1, 2, 1, 3, 2]), vec![3, 1, 2]);
}

#[test]
fn test_unique_booleans() {
    assert_eq!(
        umt_unique(&[true, false, true, false, true]),
        vec![true, false]
    );
}

#[test]
fn test_unique_f64() {
    assert_eq!(umt_unique_f64(&[1.0, 2.0, 2.0, 3.0]), vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_unique_f64_with_nan() {
    let arr = vec![1.0, f64::NAN, 2.0, f64::NAN, 3.0];
    let result = umt_unique_f64(&arr);
    // NaN values are treated as equal - only one should remain
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 1.0);
    assert!(result[1].is_nan());
    assert_eq!(result[2], 2.0);
    assert_eq!(result[3], 3.0);
}

#[test]
fn test_unique_single_element() {
    assert_eq!(umt_unique(&[42]), vec![42]);
}

#[test]
fn test_unique_large_array() {
    let arr: Vec<i32> = (0..1000).flat_map(|x| vec![x, x]).collect();
    let result = umt_unique(&arr);
    assert_eq!(result.len(), 1000);
    for i in 0..1000 {
        assert_eq!(result[i], i as i32);
    }
}
