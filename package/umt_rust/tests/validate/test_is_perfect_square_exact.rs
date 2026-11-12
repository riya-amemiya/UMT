//! Tests for exact integer and double precision perfect square checks

use umt_rust::validate::{umt_is_perfect_square, umt_is_perfect_square_f64};

#[test]
fn test_is_perfect_square_is_exact_for_large_integers() {
    let root = 3_037_000_499_i64;
    assert!(umt_is_perfect_square(root * root));
    assert!(!umt_is_perfect_square(root * root - 1));
    assert!(!umt_is_perfect_square(root * root + 1));
    assert!(!umt_is_perfect_square(i64::MAX));
}

#[test]
fn test_is_perfect_square_is_exact_above_double_precision() {
    let root = 94_906_267_i64;
    assert!(umt_is_perfect_square(root * root));
    assert!(!umt_is_perfect_square(root * root + 1));
    assert!(!umt_is_perfect_square(root * root - 1));
}

#[test]
fn test_is_perfect_square_rejects_minimum_integer() {
    assert!(!umt_is_perfect_square(i64::MIN));
}

#[test]
fn test_is_perfect_square_f64_follows_double_precision_square_root() {
    assert!(umt_is_perfect_square_f64(1e300));
    assert!(umt_is_perfect_square_f64(4.000000000000001));
    assert!(!umt_is_perfect_square_f64(0.25));
    assert!(!umt_is_perfect_square_f64(20.0));
}

#[test]
fn test_is_perfect_square_f64_non_finite_values() {
    assert!(umt_is_perfect_square_f64(f64::INFINITY));
    assert!(!umt_is_perfect_square_f64(f64::NEG_INFINITY));
    assert!(!umt_is_perfect_square_f64(f64::NAN));
}

#[test]
fn test_is_perfect_square_f64_signed_zero() {
    assert!(umt_is_perfect_square_f64(-0.0));
    assert!(umt_is_perfect_square_f64(0.0));
}
