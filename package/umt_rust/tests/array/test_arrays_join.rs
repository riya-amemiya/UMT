use umt_rust::array::{umt_arrays_join, umt_arrays_join_f64, umt_arrays_join_two};

#[test]
fn test_arrays_join_basic() {
    assert_eq!(
        umt_arrays_join(&[&[1, 2, 3][..], &[4, 5, 6][..]]),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn test_arrays_join_multiple() {
    assert_eq!(
        umt_arrays_join(&[&[1, 2, 3][..], &[4, 5, 6][..], &[7, 8, 9][..]]),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}

#[test]
fn test_arrays_join_with_duplicates() {
    assert_eq!(
        umt_arrays_join(&[&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..]]),
        vec![1, 2, 3, 4, 5]
    );
}

#[test]
fn test_arrays_join_empty() {
    let empty: &[i32] = &[];
    assert_eq!(umt_arrays_join(&[empty, &[1, 2, 3][..]]), vec![1, 2, 3]);
    assert_eq!(umt_arrays_join(&[&[1, 2, 3][..], empty]), vec![1, 2, 3]);
}

#[test]
fn test_arrays_join_strings() {
    assert_eq!(
        umt_arrays_join(&[&["a", "b", "c"][..], &["b", "c", "d"][..]]),
        vec!["a", "b", "c", "d"]
    );
    assert_eq!(
        umt_arrays_join(&[
            &["hello", "world"][..],
            &["world", "foo"][..],
            &["bar", "hello"][..]
        ]),
        vec!["hello", "world", "foo", "bar"]
    );
}

#[test]
fn test_arrays_join_two_convenience() {
    assert_eq!(
        umt_arrays_join_two(&[1, 2, 3], &[2, 3, 4]),
        vec![1, 2, 3, 4]
    );
}

#[test]
fn test_arrays_join_preserves_order() {
    assert_eq!(
        umt_arrays_join(&[&[3, 1, 2][..], &[2, 4, 1][..]]),
        vec![3, 1, 2, 4]
    );
    assert_eq!(
        umt_arrays_join(&[&["c", "a", "b"][..], &["b", "d", "a"][..]]),
        vec!["c", "a", "b", "d"]
    );
}

#[test]
fn test_arrays_join_single_array() {
    assert_eq!(umt_arrays_join(&[&[1, 2, 3][..]]), vec![1, 2, 3]);
}

#[test]
fn test_arrays_join_single_with_duplicates() {
    assert_eq!(umt_arrays_join(&[&[1, 1, 2, 2, 3, 3][..]]), vec![1, 2, 3]);
}

#[test]
fn test_arrays_join_f64_with_nan() {
    let arr1 = [1.0, f64::NAN, 2.0];
    let arr2 = [f64::NAN, 3.0];
    let result = umt_arrays_join_f64(&[&arr1[..], &arr2[..]]);

    // NaN values are deduplicated
    assert_eq!(result.len(), 4);
    assert_eq!(result[0], 1.0);
    assert!(result[1].is_nan());
    assert_eq!(result[2], 2.0);
    assert_eq!(result[3], 3.0);
}

#[test]
fn test_arrays_join_large_arrays() {
    let large_array1: Vec<i32> = (0..10_000).collect();
    let large_array2: Vec<i32> = (5000..15_000).collect();
    let result = umt_arrays_join(&[&large_array1[..], &large_array2[..]]);

    assert_eq!(result.len(), 15_000);
    assert_eq!(result[0], 0);
    assert_eq!(result[result.len() - 1], 14_999);
}

#[test]
fn test_arrays_join_owned_strings() {
    let first = ["a".to_string(), "\u{3042}".to_string()];
    let second = ["\u{3042}".to_string(), "b".to_string()];
    assert_eq!(
        umt_arrays_join(&[&first[..], &second[..]]),
        vec!["a".to_string(), "\u{3042}".to_string(), "b".to_string()]
    );
}

#[test]
fn test_arrays_join_f64_nan_and_signed_zero() {
    let first = [f64::NAN, 0.0, 1.5];
    let second = [-0.0, f64::NAN, 1.5, -2.0];
    let result = umt_arrays_join_f64(&[&first[..], &second[..]]);
    assert_eq!(result.len(), 4);
    assert!(result[0].is_nan());
    assert!(result[1] == 0.0 && result[1].is_sign_positive());
    assert_eq!(result[2], 1.5);
    assert_eq!(result[3], -2.0);
}

#[test]
fn test_arrays_join_f64_large_arrays() {
    let first: Vec<f64> = (0..20_000).map(f64::from).collect();
    let second: Vec<f64> = (10_000..30_000).map(f64::from).collect();
    let result = umt_arrays_join_f64(&[&first[..], &second[..]]);
    assert_eq!(result.len(), 30_000);
    assert_eq!(result[0], 0.0);
    assert_eq!(result[29_999], 29_999.0);
}

#[test]
fn test_arrays_join_f64_empty() {
    let empty: &[&[f64]] = &[];
    assert_eq!(umt_arrays_join_f64(empty), Vec::<f64>::new());
}

#[test]
fn test_arrays_join_f64_negative_zero_becomes_positive_zero() {
    let first = [-0.0, 2.0];
    let result = umt_arrays_join_f64(&[&first[..]]);
    assert_eq!(result.len(), 2);
    assert!(result[0] == 0.0 && result[0].is_sign_positive());
    assert_eq!(result[1], 2.0);
}
