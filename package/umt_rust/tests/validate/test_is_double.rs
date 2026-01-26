//! Tests for is_double function

use umt_rust::validate::{umt_is_double, umt_is_double_str};

#[test]
fn test_is_double_returns_true_for_valid_doubles() {
    assert!(umt_is_double(1.5));
    assert!(umt_is_double(-1.5));
    assert!(umt_is_double(1.23e-4));
    assert!(umt_is_double(0.1));
    assert!(umt_is_double(2.22));
}

#[test]
fn test_is_double_returns_false_for_integers() {
    assert!(!umt_is_double(1.0));
    assert!(!umt_is_double(0.0));
    assert!(!umt_is_double(-3.0));
    assert!(!umt_is_double(100.0));
}

#[test]
fn test_is_double_returns_false_for_special_values() {
    assert!(!umt_is_double(f64::NAN));
    assert!(!umt_is_double(f64::INFINITY));
    assert!(!umt_is_double(f64::NEG_INFINITY));
}

#[test]
fn test_is_double_str_returns_true_for_valid_double_strings() {
    assert!(umt_is_double_str("1.5"));
    assert!(umt_is_double_str("-1.5"));
    assert!(umt_is_double_str("0.1"));
    assert!(umt_is_double_str("3.14"));
}

#[test]
fn test_is_double_str_returns_false_for_integer_strings() {
    assert!(!umt_is_double_str("1"));
    assert!(!umt_is_double_str("5"));
    assert!(!umt_is_double_str("5.0"));
    assert!(!umt_is_double_str("0"));
}

#[test]
fn test_is_double_str_returns_false_for_invalid_strings() {
    assert!(!umt_is_double_str("abc"));
    assert!(!umt_is_double_str(""));
    assert!(!umt_is_double_str("NaN"));
}

#[test]
fn test_is_double_hex_value() {
    // 0x12 = 18 which is an integer
    assert!(!umt_is_double(0x12 as f64));
}

use umt_rust::validate::*;

#[test]
fn test_is_double() {
    assert!(umt_is_double(0.1));
    assert!(umt_is_double(3.14));
    assert!(umt_is_double(-2.5));
    assert!(!umt_is_double(5.0));
    assert!(!umt_is_double(0.0));
    assert!(!umt_is_double(-3.0));
    assert!(!umt_is_double(f64::NAN));
    assert!(!umt_is_double(f64::INFINITY));
}

#[test]
fn test_is_double_str() {
    assert!(umt_is_double_str("0.1"));
    assert!(umt_is_double_str("3.14"));
    assert!(!umt_is_double_str("5"));
    assert!(!umt_is_double_str("5.0"));
    assert!(!umt_is_double_str("abc"));
    assert!(!umt_is_double_str(""));
}
