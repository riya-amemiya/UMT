use umt_rust::array::{umt_unzip, umt_unzip_many, umt_unzip3, umt_zip, umt_zip_many, umt_zip3};

#[test]
fn test_unzip_pairs() {
    assert_eq!(
        umt_unzip(&[(1, "a"), (2, "b"), (3, "c")]),
        (vec![1, 2, 3], vec!["a", "b", "c"])
    );
}

#[test]
fn test_unzip_is_inverse_of_zip() {
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];
    assert_eq!(umt_unzip(&umt_zip(&numbers, &letters)), (numbers, letters));
}

#[test]
fn test_unzip_empty() {
    let empty: Vec<(i32, char)> = vec![];
    assert_eq!(umt_unzip(&empty), (vec![], vec![]));
}

#[test]
fn test_unzip3_is_inverse_of_zip3() {
    let numbers = vec![1, 2];
    let letters = vec!['a', 'b'];
    let flags = vec![true, false];
    assert_eq!(
        umt_unzip3(&umt_zip3(&numbers, &letters, &flags)),
        (numbers, letters, flags)
    );
}

#[test]
fn test_unzip3_empty() {
    let empty: Vec<(i32, char, bool)> = vec![];
    assert_eq!(umt_unzip3(&empty), (vec![], vec![], vec![]));
}

#[test]
fn test_unzip_many_is_inverse_of_zip_many() {
    let first = [1, 2, 3];
    let second = [4, 5, 6];
    assert_eq!(
        umt_unzip_many(&umt_zip_many(&[&first[..], &second[..]])),
        vec![first.to_vec(), second.to_vec()]
    );
}

#[test]
fn test_unzip_many_truncates_to_shortest_row() {
    let rows = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8]];
    assert_eq!(umt_unzip_many(&rows), vec![vec![1, 4, 6], vec![2, 5, 7]]);
}

#[test]
fn test_unzip_many_accepts_slices() {
    let rows: [&[i32]; 2] = [&[1, 2], &[3, 4]];
    assert_eq!(umt_unzip_many(&rows), vec![vec![1, 3], vec![2, 4]]);
}

#[test]
fn test_unzip_many_empty() {
    let empty: Vec<Vec<i32>> = vec![];
    assert!(umt_unzip_many(&empty).is_empty());
    assert!(umt_unzip_many(&[vec![1], vec![]]).is_empty());
}
