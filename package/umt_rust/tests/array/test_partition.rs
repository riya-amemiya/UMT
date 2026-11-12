use umt_rust::array::{umt_partition, umt_partition_indexed};

#[test]
fn test_partition_even_and_odd() {
    let (pass, fail) = umt_partition(&[1, 2, 3, 4], |n| n % 2 == 0);
    assert_eq!(pass, vec![2, 4]);
    assert_eq!(fail, vec![1, 3]);
}

#[test]
fn test_partition_always_true() {
    let (pass, fail) = umt_partition(&[1, 2, 3], |_| true);
    assert_eq!(pass, vec![1, 2, 3]);
    assert_eq!(fail, Vec::<i32>::new());
}

#[test]
fn test_partition_always_false() {
    let (pass, fail) = umt_partition(&[1, 2, 3], |_| false);
    assert_eq!(pass, Vec::<i32>::new());
    assert_eq!(fail, vec![1, 2, 3]);
}

#[test]
fn test_partition_empty() {
    let array: Vec<i32> = vec![];
    let (pass, fail) = umt_partition(&array, |_| true);
    assert!(pass.is_empty());
    assert!(fail.is_empty());
}

#[test]
fn test_partition_does_not_mutate_input() {
    let input = vec![1, 2, 3, 4];
    let _ = umt_partition(&input, |n| *n > 2);
    assert_eq!(input, vec![1, 2, 3, 4]);
}

#[test]
fn test_partition_single_element() {
    assert_eq!(umt_partition(&[1], |n| *n == 1), (vec![1], vec![]));
    assert_eq!(umt_partition(&[1], |n| *n == 0), (vec![], vec![1]));
}

#[test]
fn test_partition_strings() {
    let array = vec!["apple".to_string(), String::new(), "\u{3042}".to_string()];
    let (pass, fail) = umt_partition(&array, |s| !s.is_empty());
    assert_eq!(pass, vec!["apple".to_string(), "\u{3042}".to_string()]);
    assert_eq!(fail, vec![String::new()]);
}

#[test]
fn test_partition_nan_values() {
    let (pass, fail) = umt_partition(&[1.0, f64::NAN, -2.0], |x| *x > 0.0);
    assert_eq!(pass, vec![1.0]);
    assert_eq!(fail.len(), 2);
    assert!(fail[0].is_nan());
    assert_eq!(fail[1], -2.0);
}

#[test]
fn test_partition_indexed_receives_index() {
    let (pass, fail) = umt_partition_indexed(&["a", "b", "c"], |_, index| index % 2 == 0);
    assert_eq!(pass, vec!["a", "c"]);
    assert_eq!(fail, vec!["b"]);
}

#[test]
fn test_partition_indexed_receives_value() {
    let (pass, fail) = umt_partition_indexed(&[5, 0, 7], |value, index| *value > index as i32);
    assert_eq!(pass, vec![5, 7]);
    assert_eq!(fail, vec![0]);
}

#[test]
fn test_partition_indexed_empty() {
    let array: Vec<i32> = vec![];
    let (pass, fail) = umt_partition_indexed(&array, |_, _| true);
    assert!(pass.is_empty());
    assert!(fail.is_empty());
}
