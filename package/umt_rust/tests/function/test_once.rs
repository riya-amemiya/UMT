use std::cell::Cell;
use umt_rust::function::{umt_once, umt_once1, umt_once2};

#[test]
fn test_once_invokes_function_only_once() {
    let call_count = Cell::new(0);
    let once_fn = umt_once(|| {
        call_count.set(call_count.get() + 1);
        42
    });

    assert_eq!(once_fn(), 42);
    assert_eq!(once_fn(), 42);
    assert_eq!(once_fn(), 42);
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_once2_passes_arguments_to_first_call() {
    let call_count = Cell::new(0);
    let once_fn = umt_once2(|a: i32, b: i32| {
        call_count.set(call_count.get() + 1);
        a + b
    });

    assert_eq!(once_fn(3, 4), 7);
    assert_eq!(once_fn(10, 20), 7);
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_once1_returns_first_result_with_different_arguments() {
    let once_fn = umt_once1(|x: String| x.to_uppercase());

    assert_eq!(once_fn("hello".to_string()), "HELLO");
    assert_eq!(once_fn("world".to_string()), "HELLO");
}

#[test]
fn test_once_handles_unit_return() {
    let counter = Cell::new(0);
    let once_fn = umt_once(|| {
        counter.set(counter.get() + 1);
    });

    once_fn();
    once_fn();
    once_fn();
    assert_eq!(counter.get(), 1);
}

#[test]
fn test_once_with_string_result() {
    let once_fn = umt_once(|| String::from("initialized"));

    assert_eq!(once_fn(), "initialized");
    assert_eq!(once_fn(), "initialized");
}

#[test]
fn test_once_with_float_result() {
    let call_count = Cell::new(0);
    let once_fn = umt_once(|| {
        call_count.set(call_count.get() + 1);
        3.14_f64
    });

    assert!((once_fn() - 3.14).abs() < f64::EPSILON);
    assert!((once_fn() - 3.14).abs() < f64::EPSILON);
    assert_eq!(call_count.get(), 1);
}
