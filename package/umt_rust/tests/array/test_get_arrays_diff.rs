use umt_rust::array::{umt_get_arrays_diff, umt_get_arrays_diff_two};

#[test]
fn test_get_arrays_diff_basic() {
    assert_eq!(
        umt_get_arrays_diff(&[&[1, 2, 3][..], &[2, 3, 4][..], &[3, 4, 5][..]]),
        vec![1, 5]
    );
}

#[test]
fn test_get_arrays_diff_negative_numbers() {
    assert_eq!(
        umt_get_arrays_diff(&[&[-1, 0, 1][..], &[0, 1, 2][..], &[1, 2, 3][..]]),
        vec![-1, 3]
    );
}

#[test]
fn test_get_arrays_diff_strings() {
    assert_eq!(
        umt_get_arrays_diff(&[&["a", "b", "c"][..], &["b", "c", "d"][..], &["c", "d", "e"][..]]),
        vec!["a", "e"]
    );
}

#[test]
fn test_get_arrays_diff_empty_arrays() {
    let empty: &[i32] = &[];
    assert_eq!(
        umt_get_arrays_diff(&[empty, &[1, 2][..], &[2, 3][..]]),
        vec![1, 3]
    );
    assert_eq!(
        umt_get_arrays_diff(&[&[1, 2][..], empty, &[2, 3][..]]),
        vec![1, 3]
    );
    assert_eq!(
        umt_get_arrays_diff(&[empty, empty, &[1, 2, 3][..]]),
        vec![1, 2, 3]
    );
    assert_eq!(umt_get_arrays_diff(&[empty, empty, empty]), Vec::<i32>::new());
}

#[test]
fn test_get_arrays_diff_no_duplicates() {
    assert_eq!(
        umt_get_arrays_diff(&[&[1, 2][..], &[3, 4][..], &[5, 6][..]]),
        vec![1, 2, 3, 4, 5, 6]
    );
}

#[test]
fn test_get_arrays_diff_all_identical() {
    assert_eq!(
        umt_get_arrays_diff(&[&[1, 2, 3][..], &[1, 2, 3][..], &[1, 2, 3][..]]),
        Vec::<i32>::new()
    );
}

#[test]
fn test_get_arrays_diff_different_order() {
    assert_eq!(
        umt_get_arrays_diff(&[&[1, 2, 3][..], &[3, 1, 2][..], &[2, 3, 1][..]]),
        Vec::<i32>::new()
    );
}

#[test]
fn test_get_arrays_diff_booleans() {
    assert_eq!(
        umt_get_arrays_diff(&[&[true, false][..], &[false, true][..], &[true, false][..]]),
        Vec::<bool>::new()
    );
}

#[test]
fn test_get_arrays_diff_two_convenience() {
    assert_eq!(umt_get_arrays_diff_two(&[1, 2, 3], &[2, 3, 4]), vec![1, 4]);
}

#[test]
fn test_get_arrays_diff_single_array() {
    assert_eq!(umt_get_arrays_diff(&[&[1, 2, 3][..]]), vec![1, 2, 3]);
}

#[test]
fn test_get_arrays_diff_empty() {
    let empty: &[&[i32]] = &[];
    assert_eq!(umt_get_arrays_diff(empty), Vec::<i32>::new());
}
