use umt_rust::array::{umt_zip_longest, umt_zip_longest3, umt_zip_longest_many};

#[test]
fn test_zip_longest_same_length() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    assert_eq!(
        umt_zip_longest(&a, &b),
        vec![(Some(1), Some(4)), (Some(2), Some(5)), (Some(3), Some(6))]
    );
}

#[test]
fn test_zip_longest_first_shorter() {
    let a = vec![1, 2];
    let b = vec![4, 5, 6];
    assert_eq!(
        umt_zip_longest(&a, &b),
        vec![(Some(1), Some(4)), (Some(2), Some(5)), (None, Some(6))]
    );
}

#[test]
fn test_zip_longest_second_shorter() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5];
    assert_eq!(
        umt_zip_longest(&a, &b),
        vec![(Some(1), Some(4)), (Some(2), Some(5)), (Some(3), None)]
    );
}

#[test]
fn test_zip_longest_first_empty() {
    let a: Vec<i32> = vec![];
    let b = vec![4, 5, 6];
    assert_eq!(
        umt_zip_longest(&a, &b),
        vec![(None, Some(4)), (None, Some(5)), (None, Some(6))]
    );
}

#[test]
fn test_zip_longest_second_empty() {
    let a = vec![1, 2, 3];
    let b: Vec<i32> = vec![];
    assert_eq!(
        umt_zip_longest(&a, &b),
        vec![(Some(1), None), (Some(2), None), (Some(3), None)]
    );
}

#[test]
fn test_zip_longest_both_empty() {
    let a: Vec<i32> = vec![];
    let b: Vec<i32> = vec![];
    assert_eq!(umt_zip_longest(&a, &b), Vec::<(Option<i32>, Option<i32>)>::new());
}

#[test]
fn test_zip_longest3() {
    let a = vec![1, 2];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9, 10];
    assert_eq!(
        umt_zip_longest3(&a, &b, &c),
        vec![
            (Some(1), Some(4), Some(7)),
            (Some(2), Some(5), Some(8)),
            (None, Some(6), Some(9)),
            (None, None, Some(10)),
        ]
    );
}

#[test]
fn test_zip_longest_many() {
    let a = vec![1, 2];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9, 10];
    assert_eq!(
        umt_zip_longest_many(&[&a[..], &b[..], &c[..]]),
        vec![
            vec![Some(1), Some(4), Some(7)],
            vec![Some(2), Some(5), Some(8)],
            vec![None, Some(6), Some(9)],
            vec![None, None, Some(10)],
        ]
    );
}

#[test]
fn test_zip_longest_many_empty() {
    let arrays: &[&[i32]] = &[];
    assert_eq!(umt_zip_longest_many(arrays), Vec::<Vec<Option<i32>>>::new());
}

#[test]
fn test_zip_longest_many_single() {
    let a = vec![1, 2, 3];
    assert_eq!(
        umt_zip_longest_many(&[&a[..]]),
        vec![vec![Some(1)], vec![Some(2)], vec![Some(3)]]
    );
}

#[test]
fn test_zip_longest_with_strings() {
    let a = vec!["a", "b"];
    let b = vec!["c", "d", "e"];
    assert_eq!(
        umt_zip_longest(&a, &b),
        vec![
            (Some("a"), Some("c")),
            (Some("b"), Some("d")),
            (None, Some("e")),
        ]
    );
}
