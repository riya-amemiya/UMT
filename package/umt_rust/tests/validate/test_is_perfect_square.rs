//! Tests for is_perfect_square function

use umt_rust::validate::{umt_is_perfect_square, umt_is_perfect_square_f64};

#[test]
fn test_is_perfect_square_returns_true_for_perfect_squares() {
    assert!(umt_is_perfect_square(16));
    assert!(umt_is_perfect_square(25));
    assert!(umt_is_perfect_square(49));
    assert!(umt_is_perfect_square(0));
}

#[test]
fn test_is_perfect_square_returns_false_for_non_perfect_squares() {
    assert!(!umt_is_perfect_square(20));
    assert!(!umt_is_perfect_square(-16));
}

#[test]
fn test_is_perfect_square_returns_true_for_one() {
    assert!(umt_is_perfect_square(1));
}

#[test]
fn test_is_perfect_square_handles_large_numbers() {
    assert!(umt_is_perfect_square(100_000_000));
    assert!(!umt_is_perfect_square(100_000_002));
}

#[test]
fn test_is_perfect_square_f64_integer_values() {
    assert!(umt_is_perfect_square_f64(4.0)); // 4.0 is treated as integer 4
    assert!(umt_is_perfect_square_f64(16.0));
    assert!(umt_is_perfect_square_f64(25.0));
}

#[test]
fn test_is_perfect_square_f64_non_integer_values() {
    assert!(!umt_is_perfect_square_f64(4.5));
    assert!(!umt_is_perfect_square_f64(2.25));
}

#[test]
fn test_is_perfect_square_f64_negative_values() {
    assert!(!umt_is_perfect_square_f64(-4.0));
    assert!(!umt_is_perfect_square_f64(-16.0));
}

#[test]
fn test_is_perfect_square_zero() {
    assert!(umt_is_perfect_square(0));
    assert!(umt_is_perfect_square_f64(0.0));
}

#[test]
fn test_is_perfect_square_negative_numbers() {
    assert!(!umt_is_perfect_square(-1));
    assert!(!umt_is_perfect_square(-4));
    assert!(!umt_is_perfect_square(-100));
}
