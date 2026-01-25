use umt_rust::array::{umt_get_arrays_common, umt_get_arrays_common_f64, umt_get_arrays_common_two};

#[test]
fn test_get_arrays_common_basic() {
    assert_eq!(
        umt_get_arrays_common(&[&[1, 2, 3][..], &[2, 3, 4][..]]),
        vec![2, 3]
    );
}

#[test]
fn test_get_arrays_common_three_arrays() {
    assert_eq!(
        umt_get_arrays_common(&[&[1, 2, 3][..], &[2, 3, 9][..], &[3, 4, 5][..]]),
        vec![3]
    );
}

#[test]
fn test_get_arrays_common_four_arrays() {
    assert_eq!(
        umt_get_arrays_common(&[&[1, 2, 3][..], &[2, 3, 9][..], &[3, 4, 5][..], &[3, 4, 5][..]]),
        vec![3]
    );
}

#[test]
fn test_get_arrays_common_empty() {
    let empty: &[i32] = &[];
    assert_eq!(umt_get_arrays_common(&[empty, empty]), Vec::<i32>::new());
    assert_eq!(umt_get_arrays_common(&[&[1, 2, 3][..], empty]), Vec::<i32>::new());
    assert_eq!(umt_get_arrays_common(&[empty, &[1, 2, 3][..]]), Vec::<i32>::new());
}

#[test]
fn test_get_arrays_common_no_common() {
    assert_eq!(
        umt_get_arrays_common(&[&[1, 2, 3][..], &[4, 5, 6][..]]),
        Vec::<i32>::new()
    );
    assert_eq!(
        umt_get_arrays_common(&[&[1, 2][..], &[3, 4][..], &[5, 6][..]]),
        Vec::<i32>::new()
    );
}

#[test]
fn test_get_arrays_common_strings() {
    assert_eq!(
        umt_get_arrays_common(&[&["a", "b", "c"][..], &["b", "c", "d"][..]]),
        vec!["b", "c"]
    );
    assert_eq!(
        umt_get_arrays_common(&[&["a", "b", "c"][..], &["b", "c", "d"][..], &["c", "d", "e"][..]]),
        vec!["c"]
    );
}

#[test]
fn test_get_arrays_common_single_array() {
    assert_eq!(umt_get_arrays_common(&[&[1, 2, 3][..]]), vec![1, 2, 3]);
    assert_eq!(umt_get_arrays_common(&[&["a", "b", "c"][..]]), vec!["a", "b", "c"]);
    let empty: &[&[i32]] = &[];
    assert_eq!(umt_get_arrays_common(empty), Vec::<i32>::new());
}

#[test]
fn test_get_arrays_common_with_duplicates() {
    assert_eq!(
        umt_get_arrays_common(&[&[1, 1, 2, 2, 3][..], &[2, 2, 3, 3, 4][..], &[3, 3, 4, 4, 5][..]]),
        vec![3]
    );
}

#[test]
fn test_get_arrays_common_preserves_order() {
    assert_eq!(
        umt_get_arrays_common(&[&[3, 1, 2][..], &[2, 3, 1][..], &[1, 3, 2][..]]),
        vec![3, 1, 2]
    );
    assert_eq!(
        umt_get_arrays_common(&[&["c", "a", "b"][..], &["b", "c", "a"][..], &["a", "c", "b"][..]]),
        vec!["c", "a", "b"]
    );
}

#[test]
fn test_get_arrays_common_booleans() {
    assert_eq!(
        umt_get_arrays_common(&[&[true, false, true][..], &[false, true][..], &[true][..]]),
        vec![true]
    );
    assert_eq!(
        umt_get_arrays_common(&[&[true, false][..], &[false, true][..], &[true, false][..]]),
        vec![true, false]
    );
}

#[test]
fn test_get_arrays_common_two_convenience() {
    assert_eq!(umt_get_arrays_common_two(&[1, 2, 3], &[2, 3, 4]), vec![2, 3]);
}

#[test]
fn test_get_arrays_common_large_arrays() {
    let array1: Vec<i32> = (0..10_000).collect();
    let array2: Vec<i32> = (5000..15_000).collect();
    let array3: Vec<i32> = (7500..17_500).collect();

    let result = umt_get_arrays_common(&[&array1[..], &array2[..], &array3[..]]);

    assert_eq!(result.len(), 2500);
    assert_eq!(result[0], 7500);
    assert_eq!(result[result.len() - 1], 9999);
}

#[test]
fn test_get_arrays_common_f64() {
    assert_eq!(
        umt_get_arrays_common_f64(&[&[1.0, 2.0, 3.0][..], &[2.0, 3.0, 4.0][..]]),
        vec![2.0, 3.0]
    );
}

#[test]
fn test_get_arrays_common_f64_with_nan() {
    let arr1 = [f64::NAN, 1.0, 2.0];
    let arr2 = [f64::NAN, 2.0, 3.0];
    let result = umt_get_arrays_common_f64(&[&arr1[..], &arr2[..]]);

    assert_eq!(result.len(), 2);
    assert!(result[0].is_nan());
    assert_eq!(result[1], 2.0);
}
