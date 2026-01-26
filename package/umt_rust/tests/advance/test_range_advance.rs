use umt_rust::advance::{umt_range_advance, umt_range_advance_filtered};

#[test]
fn test_umt_range_advance_returns_full_range_when_no_condition() {
    let result = umt_range_advance(0, 5, None::<fn(i32) -> bool>);
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_umt_range_advance_with_start_and_end() {
    let result = umt_range_advance(2, 5, None::<fn(i32) -> bool>);
    assert_eq!(result, vec![2, 3, 4]);
}

#[test]
fn test_umt_range_advance_with_even_conditional() {
    let is_even = |num: i32| num % 2 == 0;
    let result = umt_range_advance(0, 10, Some(is_even));
    assert_eq!(result, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_umt_range_advance_empty_when_start_greater_than_end() {
    let result = umt_range_advance(5, 2, None::<fn(i32) -> bool>);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_umt_range_advance_empty_when_start_equals_end_with_unsatisfied_condition() {
    let is_odd = |num: i32| num % 2 != 0;
    let result = umt_range_advance(5, 5, Some(is_odd));
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_umt_range_advance_filtered_basic() {
    let result = umt_range_advance_filtered(0, 10, |n| n % 2 == 0);
    assert_eq!(result, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_umt_range_advance_filtered_odd_numbers() {
    let result = umt_range_advance_filtered(0, 10, |n| n % 2 != 0);
    assert_eq!(result, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_umt_range_advance_with_negative_range() {
    let result = umt_range_advance(-5, 5, Some(|n: i32| n >= 0));
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_umt_range_advance_divisible_by_three() {
    let result = umt_range_advance(1, 10, Some(|n: i32| n % 3 == 0));
    assert_eq!(result, vec![3, 6, 9]);
}

#[test]
fn test_umt_range_advance_prime_check_like() {
    // Simple filter: numbers greater than 5
    let result = umt_range_advance(1, 10, Some(|n: i32| n > 5));
    assert_eq!(result, vec![6, 7, 8, 9]);
}

use umt_rust::advance::*;

#[test]
fn test_range_advance_empty_when_start_equals_end_with_unsatisfied_condition() {
    let is_odd = |num: i32| num % 2 != 0;
    let result = umt_range_advance(5, 5, Some(is_odd));
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_range_advance_empty_when_start_greater_than_end() {
    let result = umt_range_advance(5, 2, None::<fn(i32) -> bool>);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_range_advance_filtered_divisible_by_three() {
    let result = umt_range_advance_filtered(0, 15, |n| n % 3 == 0);
    assert_eq!(result, vec![0, 3, 6, 9, 12]);
}

#[test]
fn test_range_advance_filtered_with_even() {
    let result = umt_range_advance_filtered(0, 10, |n| n % 2 == 0);
    assert_eq!(result, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_range_advance_returns_full_range_without_condition() {
    let result = umt_range_advance(0, 5, None::<fn(i32) -> bool>);
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_range_advance_with_even_condition() {
    let is_even = |num: i32| num % 2 == 0;
    let result = umt_range_advance(0, 10, Some(is_even));
    assert_eq!(result, vec![0, 2, 4, 6, 8]);
}

#[test]
fn test_range_advance_with_negative_numbers() {
    let is_negative = |num: i32| num < 0;
    let result = umt_range_advance(-5, 5, Some(is_negative));
    assert_eq!(result, vec![-5, -4, -3, -2, -1]);
}

#[test]
fn test_range_advance_with_odd_condition() {
    let is_odd = |num: i32| num % 2 != 0;
    let result = umt_range_advance(0, 10, Some(is_odd));
    assert_eq!(result, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_range_advance_with_start_and_end() {
    let result = umt_range_advance(2, 5, None::<fn(i32) -> bool>);
    assert_eq!(result, vec![2, 3, 4]);
}
