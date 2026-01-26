use umt_rust::array::{umt_zip, umt_zip_many, umt_zip3};

#[test]
fn test_zip_basic() {
    let a = vec![1, 2, 3];
    let b = vec!["a", "b", "c"];
    assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b"), (3, "c")]);
}

#[test]
fn test_zip_different_lengths_first_shorter() {
    let a = vec![1, 2];
    let b = vec!["a", "b", "c"];
    assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b")]);
}

#[test]
fn test_zip_different_lengths_second_shorter() {
    let a = vec![1, 2, 3];
    let b = vec!["a", "b"];
    assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b")]);
}

#[test]
fn test_zip_empty_arrays() {
    let a: Vec<i32> = vec![];
    let b: Vec<&str> = vec![];
    assert_eq!(umt_zip(&a, &b), Vec::<(i32, &str)>::new());
}

#[test]
fn test_zip_one_empty() {
    let a = vec![1, 2, 3];
    let b: Vec<&str> = vec![];
    assert_eq!(umt_zip(&a, &b), Vec::<(i32, &str)>::new());
}

#[test]
fn test_zip_same_type() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    assert_eq!(umt_zip(&a, &b), vec![(1, 4), (2, 5), (3, 6)]);
}

#[test]
fn test_zip_with_booleans() {
    let a = vec![1, 2];
    let b = vec![true, false];
    assert_eq!(umt_zip(&a, &b), vec![(1, true), (2, false)]);
}

#[test]
fn test_zip_with_structs() {
    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        value: i32,
    }

    let a = vec![Item { value: 1 }, Item { value: 2 }];
    let b = vec!["a", "b"];
    assert_eq!(
        umt_zip(&a, &b),
        vec![(Item { value: 1 }, "a"), (Item { value: 2 }, "b")]
    );
}

#[test]
fn test_zip3() {
    let a = vec![1, 2, 3];
    let b = vec!["a", "b", "c"];
    let c = vec![true, false, true];
    assert_eq!(
        umt_zip3(&a, &b, &c),
        vec![(1, "a", true), (2, "b", false), (3, "c", true)]
    );
}

#[test]
fn test_zip3_different_lengths() {
    let a = vec![1, 2];
    let b = vec!["a", "b", "c"];
    let c = vec![true, false, true, false];
    assert_eq!(umt_zip3(&a, &b, &c), vec![(1, "a", true), (2, "b", false)]);
}

#[test]
fn test_zip_many() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9];
    assert_eq!(
        umt_zip_many(&[&a[..], &b[..], &c[..]]),
        vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]
    );
}

#[test]
fn test_zip_many_different_lengths() {
    let a = vec![1, 2];
    let b = vec![4, 5, 6];
    let c = vec![7, 8, 9, 10];
    assert_eq!(
        umt_zip_many(&[&a[..], &b[..], &c[..]]),
        vec![vec![1, 4, 7], vec![2, 5, 8]]
    );
}

#[test]
fn test_zip_many_empty() {
    let arrays: &[&[i32]] = &[];
    assert_eq!(umt_zip_many(arrays), Vec::<Vec<i32>>::new());
}
