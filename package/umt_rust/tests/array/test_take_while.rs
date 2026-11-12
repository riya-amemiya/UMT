use std::cell::RefCell;
use umt_rust::array::{umt_take_while, umt_take_while_indexed};

#[test]
fn test_takes_leading_elements() {
    assert_eq!(umt_take_while(&[1, 2, 3, 4, 1], |n| *n < 3), vec![1, 2]);
}

#[test]
fn test_all_pass() {
    assert_eq!(umt_take_while(&[1, 2, 3], |_| true), vec![1, 2, 3]);
}

#[test]
fn test_first_fails() {
    assert_eq!(umt_take_while(&[5, 1, 2], |n| *n < 3), Vec::<i32>::new());
}

#[test]
fn test_empty_input() {
    let empty: Vec<i32> = vec![];
    assert!(umt_take_while(&empty, |_| true).is_empty());
}

#[test]
fn test_indexed_receives_value_and_index() {
    let calls = RefCell::new(Vec::new());
    let result = umt_take_while_indexed(&['a', 'b', 'c'], |value, index| {
        calls.borrow_mut().push((*value, index));
        index < 1
    });
    assert_eq!(result, vec!['a']);
    assert_eq!(calls.into_inner(), vec![('a', 0), ('b', 1)]);
}

#[test]
fn test_indexed_all_pass() {
    assert_eq!(umt_take_while_indexed(&[5, 6], |_, _| true), vec![5, 6]);
}

#[test]
fn test_does_not_mutate_input() {
    let input = vec![1, 2, 3];
    let _ = umt_take_while(&input, |n| *n < 2);
    assert_eq!(input, vec![1, 2, 3]);
}
