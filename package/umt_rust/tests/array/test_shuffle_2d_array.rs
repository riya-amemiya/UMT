use std::collections::HashSet;
use umt_rust::array::umt_shuffle_2d_array;

#[test]
fn test_shuffle_2d_maintains_row_lengths() {
    let arr = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let shuffled = umt_shuffle_2d_array(&arr);

    assert_eq!(shuffled.len(), 3);
    assert_eq!(shuffled[0].len(), 2);
    assert_eq!(shuffled[1].len(), 3);
    assert_eq!(shuffled[2].len(), 1);
}

#[test]
fn test_shuffle_2d_maintains_elements() {
    let arr = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let shuffled = umt_shuffle_2d_array(&arr);

    let original_elements: HashSet<_> = arr.iter().flatten().cloned().collect();
    let shuffled_elements: HashSet<_> = shuffled.iter().flatten().cloned().collect();

    assert_eq!(original_elements, shuffled_elements);
}

#[test]
fn test_shuffle_2d_empty() {
    let arr: Vec<Vec<i32>> = vec![];
    let shuffled = umt_shuffle_2d_array(&arr);
    assert!(shuffled.is_empty());
}

#[test]
fn test_shuffle_2d_single_row() {
    let arr = vec![vec![1, 2, 3, 4, 5]];
    let shuffled = umt_shuffle_2d_array(&arr);

    assert_eq!(shuffled.len(), 1);
    assert_eq!(shuffled[0].len(), 5);

    let original_elements: HashSet<_> = arr[0].iter().cloned().collect();
    let shuffled_elements: HashSet<_> = shuffled[0].iter().cloned().collect();
    assert_eq!(original_elements, shuffled_elements);
}

#[test]
fn test_shuffle_2d_single_column() {
    let arr = vec![vec![1], vec![2], vec![3], vec![4], vec![5]];
    let shuffled = umt_shuffle_2d_array(&arr);

    assert_eq!(shuffled.len(), 5);
    for row in &shuffled {
        assert_eq!(row.len(), 1);
    }

    let original_elements: HashSet<_> = arr.iter().flatten().cloned().collect();
    let shuffled_elements: HashSet<_> = shuffled.iter().flatten().cloned().collect();
    assert_eq!(original_elements, shuffled_elements);
}

#[test]
fn test_shuffle_2d_empty_rows() {
    let arr = vec![vec![1, 2], vec![], vec![3, 4, 5]];
    let shuffled = umt_shuffle_2d_array(&arr);

    assert_eq!(shuffled.len(), 3);
    assert_eq!(shuffled[0].len(), 2);
    assert_eq!(shuffled[1].len(), 0);
    assert_eq!(shuffled[2].len(), 3);
}

#[test]
fn test_shuffle_2d_strings() {
    let arr = vec![vec!["a", "b"], vec!["c", "d"], vec!["e", "f"]];
    let shuffled = umt_shuffle_2d_array(&arr);

    let original_elements: HashSet<_> = arr.iter().flatten().cloned().collect();
    let shuffled_elements: HashSet<_> = shuffled.iter().flatten().cloned().collect();
    assert_eq!(original_elements, shuffled_elements);
}

#[test]
fn test_shuffle_2d_equal_rows() {
    let arr = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let shuffled = umt_shuffle_2d_array(&arr);

    assert_eq!(shuffled.len(), 3);
    for row in &shuffled {
        assert_eq!(row.len(), 3);
    }
}

#[test]
fn test_shuffle_2d_single_element() {
    let arr = vec![vec![42]];
    let shuffled = umt_shuffle_2d_array(&arr);

    assert_eq!(shuffled.len(), 1);
    assert_eq!(shuffled[0].len(), 1);
    assert_eq!(shuffled[0][0], 42);
}
