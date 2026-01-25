//! Tests for is_number function

use umt_rust::validate::{umt_is_number, umt_is_number_i64, umt_is_number_str};

// Integer and decimal tests
#[test]
fn test_is_number_returns_true_for_integers() {
    assert!(umt_is_number(5.0));
    assert!(umt_is_number(-5.0));
    assert!(umt_is_number(0.0));
}

#[test]
fn test_is_number_returns_true_for_decimals() {
    assert!(umt_is_number(5.5));
    assert!(umt_is_number(-5.5));
    assert!(umt_is_number(0.1));
}

// String representation tests (loose mode)
#[test]
fn test_is_number_str_returns_true_for_valid_number_strings() {
    assert!(umt_is_number_str("5"));
    assert!(umt_is_number_str("-5.5"));
    assert!(umt_is_number_str("0"));
    assert!(umt_is_number_str("0.1"));
    assert!(umt_is_number_str("42"));
    assert!(umt_is_number_str("-123.45"));
}

#[test]
fn test_is_number_str_returns_false_for_invalid_number_strings() {
    assert!(!umt_is_number_str("abc"));
    assert!(!umt_is_number_str("5abc"));
    assert!(!umt_is_number_str("NaN"));
}

#[test]
fn test_is_number_str_returns_false_for_empty_string() {
    // In Rust, empty string is not a valid number
    assert!(!umt_is_number_str(""));
}

// Special number values
#[test]
fn test_is_number_returns_false_for_nan() {
    assert!(!umt_is_number(f64::NAN));
}

#[test]
fn test_is_number_returns_false_for_infinity() {
    assert!(!umt_is_number(f64::INFINITY));
    assert!(!umt_is_number(f64::NEG_INFINITY));
}

// i64 tests
#[test]
fn test_is_number_i64_always_true() {
    assert!(umt_is_number_i64(42));
    assert!(umt_is_number_i64(-100));
    assert!(umt_is_number_i64(0));
    assert!(umt_is_number_i64(i64::MAX));
    assert!(umt_is_number_i64(i64::MIN));
}
