use std::cell::RefCell;
use std::rc::Rc;
use umt_rust::iterator::lazy_take;

#[test]
fn test_takes_first_n_elements() {
    let result: Vec<i32> = lazy_take(vec![1, 2, 3, 4, 5], 3).collect();
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_takes_all_when_n_exceeds_length() {
    let result: Vec<i32> = lazy_take(vec![1, 2], 5).collect();
    assert_eq!(result, vec![1, 2]);
}

#[test]
fn test_takes_nothing_when_n_is_0() {
    let result: Vec<i32> = lazy_take(vec![1, 2, 3], 0).collect();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_handles_empty_iterable() {
    let result: Vec<i32> = lazy_take(vec![], 5).collect();
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_evaluates_lazily_and_stops_early() {
    let call_count = Rc::new(RefCell::new(0));
    let call_count_clone = call_count.clone();

    let source = (0..100).map(move |i| {
        *call_count_clone.borrow_mut() += 1;
        i
    });

    let result: Vec<i32> = lazy_take(source, 3).collect();

    assert_eq!(result, vec![0, 1, 2]);
    assert_eq!(*call_count.borrow(), 3);
}
