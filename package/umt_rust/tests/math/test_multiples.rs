use umt_rust::math::umt_multiples;

#[test]
fn test_multiples_of_1() {
    assert_eq!(umt_multiples(1.0, 1), vec![1.0]);
    assert_eq!(umt_multiples(1.0, 3), vec![1.0, 2.0, 3.0]);
    assert_eq!(umt_multiples(1.0, 5), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
}

#[test]
fn test_multiples_of_2() {
    assert_eq!(umt_multiples(2.0, 1), vec![2.0]);
    assert_eq!(umt_multiples(2.0, 3), vec![2.0, 4.0, 6.0]);
    assert_eq!(umt_multiples(2.0, 5), vec![2.0, 4.0, 6.0, 8.0, 10.0]);
}

#[test]
fn test_multiples_of_negative() {
    assert_eq!(umt_multiples(-2.0, 3), vec![-2.0, -4.0, -6.0]);
    assert_eq!(umt_multiples(-3.0, 4), vec![-3.0, -6.0, -9.0, -12.0]);
}

#[test]
fn test_multiples_of_zero() {
    assert_eq!(umt_multiples(0.0, 3), vec![0.0, 0.0, 0.0]);
}

#[test]
fn test_multiples_zero_count() {
    let result: Vec<f64> = vec![];
    assert_eq!(umt_multiples(2.0, 0), result);
    assert_eq!(umt_multiples(-2.0, 0), result);
}

#[test]
fn test_multiples_decimals() {
    assert_eq!(umt_multiples(0.5, 3), vec![0.5, 1.0, 1.5]);
    assert_eq!(umt_multiples(1.5, 3), vec![1.5, 3.0, 4.5]);
}

use umt_rust::math::*;

#[test]
fn test_multiples_basic() {
    assert_eq!(umt_multiples(2.0, 5), vec![2.0, 4.0, 6.0, 8.0, 10.0]);
}

#[test]
fn test_multiples_float() {
    let result = umt_multiples(0.5, 4);
    assert_eq!(result, vec![0.5, 1.0, 1.5, 2.0]);
}

#[test]
fn test_multiples_negative() {
    assert_eq!(umt_multiples(-2.0, 3), vec![-2.0, -4.0, -6.0]);
}

#[test]
fn test_multiples_one() {
    assert_eq!(umt_multiples(3.0, 1), vec![3.0]);
}

#[test]
fn test_multiples_zero() {
    let result: Vec<f64> = vec![];
    assert_eq!(umt_multiples(5.0, 0), result);
}
