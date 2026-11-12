use umt_rust::array::umt_sliding;

#[test]
fn test_sliding_step_one() {
    assert_eq!(
        umt_sliding(&[1, 2, 3, 4, 5], 3, 1),
        vec![vec![1, 2, 3], vec![2, 3, 4], vec![3, 4, 5]]
    );
}

#[test]
fn test_sliding_custom_step() {
    assert_eq!(
        umt_sliding(&[1, 2, 3, 4, 5], 3, 2),
        vec![vec![1, 2, 3], vec![3, 4, 5]]
    );
}

#[test]
fn test_sliding_step_equals_size() {
    assert_eq!(
        umt_sliding(&[1, 2, 3, 4, 5, 6], 2, 2),
        vec![vec![1, 2], vec![3, 4], vec![5, 6]]
    );
}

#[test]
fn test_sliding_omits_incomplete_trailing_windows() {
    assert_eq!(umt_sliding(&[1, 2, 3, 4, 5], 3, 3), vec![vec![1, 2, 3]]);
}

#[test]
fn test_sliding_size_larger_than_length() {
    assert_eq!(umt_sliding(&[1, 2], 5, 1), Vec::<Vec<i32>>::new());
}

#[test]
fn test_sliding_empty_input() {
    let array: Vec<i32> = vec![];
    assert_eq!(umt_sliding(&array, 2, 1), Vec::<Vec<i32>>::new());
}

#[test]
fn test_sliding_size_one() {
    assert_eq!(
        umt_sliding(&[1, 2, 3], 1, 1),
        vec![vec![1], vec![2], vec![3]]
    );
}

#[test]
fn test_sliding_size_equal_to_length() {
    assert_eq!(umt_sliding(&[1, 2, 3], 3, 1), vec![vec![1, 2, 3]]);
}

#[test]
fn test_sliding_size_zero() {
    assert_eq!(
        umt_sliding(&[1, 2], 0, 1),
        vec![Vec::<i32>::new(), Vec::new(), Vec::new()]
    );
}

#[test]
fn test_sliding_step_zero() {
    assert_eq!(umt_sliding(&[1, 2, 3], 2, 0), Vec::<Vec<i32>>::new());
}

#[test]
fn test_sliding_does_not_mutate_input() {
    let input = vec![1, 2, 3, 4];
    let _ = umt_sliding(&input, 2, 1);
    assert_eq!(input, vec![1, 2, 3, 4]);
}

#[test]
fn test_sliding_strings() {
    assert_eq!(
        umt_sliding(&["a", "b", "c", "d"], 2, 1),
        vec![vec!["a", "b"], vec!["b", "c"], vec!["c", "d"]]
    );
}

#[test]
fn test_sliding_single_element() {
    assert_eq!(umt_sliding(&[42], 1, 1), vec![vec![42]]);
}
