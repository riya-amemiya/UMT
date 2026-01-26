use umt_rust::array::sorting_helpers::*;

fn default_compare<T: PartialOrd>(a: &T, b: &T) -> i32 {
    if a > b {
        1
    } else if a < b {
        -1
    } else {
        0
    }
}

#[test]
fn test_insertion_sort_range_full() {
    let mut arr = vec![5, 2, 8, 1, 9];
    insertion_sort_range(&mut arr, &default_compare, 0, 4);
    assert_eq!(arr, vec![1, 2, 5, 8, 9]);
}

#[test]
fn test_insertion_sort_range_partial() {
    let mut arr = vec![5, 2, 8, 1, 9];
    insertion_sort_range(&mut arr, &default_compare, 1, 3);
    assert_eq!(arr, vec![5, 1, 2, 8, 9]);
}

#[test]
fn test_insertion_sort_range_empty() {
    let mut arr: Vec<i32> = vec![];
    insertion_sort_range(&mut arr, &default_compare, 0, 0);
    assert_eq!(arr, Vec::<i32>::new());
}

#[test]
fn test_insertion_sort_range_single() {
    let mut arr = vec![42];
    insertion_sort_range(&mut arr, &default_compare, 0, 0);
    assert_eq!(arr, vec![42]);
}

#[test]
fn test_insertion_sort_range_already_sorted() {
    let mut arr = vec![1, 2, 3, 4, 5];
    insertion_sort_range(&mut arr, &default_compare, 0, 4);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_insertion_sort_range_reverse() {
    let mut arr = vec![5, 4, 3, 2, 1];
    insertion_sort_range(&mut arr, &default_compare, 0, 4);
    assert_eq!(arr, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_insertion_sort_range_strings() {
    let mut arr = vec!["banana", "apple", "cherry"];
    insertion_sort_range(&mut arr, &default_compare, 0, 2);
    assert_eq!(arr, vec!["apple", "banana", "cherry"]);
}
