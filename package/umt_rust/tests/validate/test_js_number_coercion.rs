//! Tests for JavaScript numeric string coercion in the string predicates

use umt_rust::validate::{umt_is_double_str, umt_is_number_str, umt_is_value_nan_str_loose};

fn with_char(code: u32, rest: &str) -> String {
    format!("{}{}", char::from_u32(code).unwrap(), rest)
}

#[test]
fn test_is_number_str_ignores_surrounding_whitespace() {
    assert!(umt_is_number_str(" 42 "));
    assert!(umt_is_number_str("\t1.5\n"));
    assert!(umt_is_number_str(&with_char(0xFEFF, "7")));
    assert!(umt_is_number_str(&with_char(0x3000, "7")));
}

#[test]
fn test_is_number_str_rejects_non_javascript_whitespace() {
    assert!(!umt_is_number_str(&with_char(0x85, "1")));
    assert!(!umt_is_number_str(&with_char(0x1C, "1")));
}

#[test]
fn test_is_number_str_accepts_empty_and_whitespace_only_strings() {
    assert!(umt_is_number_str(""));
    assert!(umt_is_number_str("   "));
    assert!(umt_is_number_str("\t\n"));
}

#[test]
fn test_is_number_str_accepts_radix_literals() {
    assert!(umt_is_number_str("0x10"));
    assert!(umt_is_number_str("0XfF"));
    assert!(umt_is_number_str("0o17"));
    assert!(umt_is_number_str("0O17"));
    assert!(umt_is_number_str("0b11"));
    assert!(umt_is_number_str("0B11"));
    assert!(umt_is_number_str(&format!("0x{}", "f".repeat(40))));
}

#[test]
fn test_is_number_str_rejects_malformed_radix_literals() {
    assert!(!umt_is_number_str("0x"));
    assert!(!umt_is_number_str("0b2"));
    assert!(!umt_is_number_str("0o8"));
    assert!(!umt_is_number_str("0xg"));
    assert!(!umt_is_number_str("-0x10"));
    assert!(!umt_is_number_str("0z1"));
    assert!(!umt_is_number_str("1x1"));
}

#[test]
fn test_is_number_str_rejects_overflowing_radix_literals() {
    assert!(!umt_is_number_str(&format!("0x{}", "f".repeat(300))));
}

#[test]
fn test_is_number_str_accepts_decimal_literals() {
    assert!(umt_is_number_str("+5"));
    assert!(umt_is_number_str("5."));
    assert!(umt_is_number_str(".5"));
    assert!(umt_is_number_str("1e3"));
    assert!(umt_is_number_str("1E+3"));
    assert!(umt_is_number_str("-1.5e-3"));
    assert!(umt_is_number_str("00012"));
}

#[test]
fn test_is_number_str_rejects_malformed_decimal_literals() {
    assert!(!umt_is_number_str("."));
    assert!(!umt_is_number_str("+"));
    assert!(!umt_is_number_str("1e"));
    assert!(!umt_is_number_str("1e+"));
    assert!(!umt_is_number_str("e5"));
    assert!(!umt_is_number_str("1.5.2"));
    assert!(!umt_is_number_str("--1"));
    assert!(!umt_is_number_str("1 2"));
    assert!(!umt_is_number_str("1_000"));
    assert!(!umt_is_number_str("12abc"));
    assert!(!umt_is_number_str(&with_char(0x0661, "")));
}

#[test]
fn test_is_number_str_rejects_non_finite_values() {
    assert!(!umt_is_number_str("Infinity"));
    assert!(!umt_is_number_str("+Infinity"));
    assert!(!umt_is_number_str("-Infinity"));
    assert!(!umt_is_number_str("1e400"));
    assert!(!umt_is_number_str("inf"));
    assert!(!umt_is_number_str("infinity"));
    assert!(!umt_is_number_str("nan"));
    assert!(!umt_is_number_str("NaN"));
}

#[test]
fn test_is_double_str_follows_javascript_coercion() {
    assert!(umt_is_double_str(" 1.5 "));
    assert!(umt_is_double_str("15e-1"));
    assert!(umt_is_double_str(".5"));
    assert!(!umt_is_double_str("1.5e1"));
    assert!(!umt_is_double_str("0x10"));
    assert!(!umt_is_double_str("   "));
    assert!(!umt_is_double_str("Infinity"));
    assert!(!umt_is_double_str("-Infinity"));
    assert!(!umt_is_double_str("inf"));
    assert!(!umt_is_double_str("1_0.5"));
}

#[test]
fn test_is_value_nan_str_loose_follows_javascript_coercion() {
    assert!(!umt_is_value_nan_str_loose(" 12 "));
    assert!(!umt_is_value_nan_str_loose("0x10"));
    assert!(!umt_is_value_nan_str_loose("-Infinity"));
    assert!(!umt_is_value_nan_str_loose("+Infinity"));
    assert!(!umt_is_value_nan_str_loose("1e400"));
    assert!(!umt_is_value_nan_str_loose("\t\n"));
    assert!(umt_is_value_nan_str_loose("inf"));
    assert!(umt_is_value_nan_str_loose("infinity"));
    assert!(umt_is_value_nan_str_loose("nan"));
    assert!(umt_is_value_nan_str_loose("1_000"));
    assert!(umt_is_value_nan_str_loose("-0x10"));
    assert!(umt_is_value_nan_str_loose("12abc"));
}
