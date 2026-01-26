//! Tests for is_value_nan function

use umt_rust::validate::{
    umt_is_value_nan, umt_is_value_nan_str, umt_is_value_nan_str_loose, umt_is_value_nan_str_strict,
};

// Strict mode tests (f64)
#[test]
fn test_strict_mode_returns_false_for_valid_numbers() {
    assert!(!umt_is_value_nan(0.0));
    assert!(!umt_is_value_nan(1.0));
    assert!(!umt_is_value_nan(-1.0));
    assert!(!umt_is_value_nan(f64::INFINITY));
}

#[test]
fn test_strict_mode_returns_true_for_nan() {
    assert!(umt_is_value_nan(f64::NAN));
}

#[test]
fn test_strict_mode_strings_returns_false() {
    // In strict mode, strings always return false
    assert!(!umt_is_value_nan_str_strict("NaN"));
    assert!(!umt_is_value_nan_str_strict("0"));
    assert!(!umt_is_value_nan_str_strict("a"));
    assert!(!umt_is_value_nan_str_strict(""));
}

// Loose mode tests (string)
#[test]
fn test_loose_mode_returns_false_for_valid_number_strings() {
    assert!(!umt_is_value_nan_str_loose("0"));
    assert!(!umt_is_value_nan_str_loose("1"));
    assert!(!umt_is_value_nan_str_loose("-1"));
    assert!(!umt_is_value_nan_str_loose("Infinity"));
}

#[test]
fn test_loose_mode_returns_true_for_nan_and_nan_strings() {
    assert!(umt_is_value_nan_str_loose("NaN"));
}

#[test]
fn test_loose_mode_handles_non_numeric_strings() {
    assert!(umt_is_value_nan_str_loose("a"));
    assert!(umt_is_value_nan_str_loose("abc"));
}

#[test]
fn test_loose_mode_handles_empty_and_whitespace_strings() {
    // Empty string coerces to 0 in JavaScript (not NaN)
    assert!(!umt_is_value_nan_str_loose(""));
    // Whitespace coerces to 0 (not NaN)
    assert!(!umt_is_value_nan_str_loose(" "));
}

// Combined mode tests
#[test]
fn test_is_value_nan_str_with_loose_true() {
    assert!(umt_is_value_nan_str("NaN", true));
    assert!(umt_is_value_nan_str("abc", true));
    assert!(!umt_is_value_nan_str("0", true));
    assert!(!umt_is_value_nan_str("1", true));
}

#[test]
fn test_is_value_nan_str_with_loose_false() {
    assert!(!umt_is_value_nan_str("NaN", false));
    assert!(!umt_is_value_nan_str("abc", false));
    assert!(!umt_is_value_nan_str("0", false));
}

#[test]
fn test_parsed_nan_value() {
    // Parsing invalid string results in NaN
    let nan_value = "not a number".parse::<f64>().unwrap_or(f64::NAN);
    assert!(umt_is_value_nan(nan_value));
}

use umt_rust::validate::*;

#[test]
fn test_is_value_nan_str_with_mode() {
    assert!(umt_is_value_nan_str("NaN", true));
    assert!(!umt_is_value_nan_str("NaN", false));
    assert!(umt_is_value_nan_str("abc", true));
    assert!(!umt_is_value_nan_str("abc", false));
}

#[test]
fn test_loose_mode_nan_strings() {
    assert!(umt_is_value_nan_str_loose("NaN"));
    assert!(umt_is_value_nan_str_loose("a"));
    assert!(umt_is_value_nan_str_loose("abc"));
}

#[test]
fn test_loose_mode_special_strings() {
    // Empty string coerces to 0 (not NaN)
    assert!(!umt_is_value_nan_str_loose(""));
    // Whitespace coerces to 0 (not NaN)
    assert!(!umt_is_value_nan_str_loose(" "));
}

#[test]
fn test_loose_mode_valid_number_strings() {
    assert!(!umt_is_value_nan_str_loose("0"));
    assert!(!umt_is_value_nan_str_loose("1"));
    assert!(!umt_is_value_nan_str_loose("-1"));
    assert!(!umt_is_value_nan_str_loose("Infinity"));
}

#[test]
fn test_strict_mode_nan() {
    assert!(umt_is_value_nan(f64::NAN));
    // Parsing invalid string results in NaN
    assert!(umt_is_value_nan(
        "not a number".parse::<f64>().unwrap_or(f64::NAN)
    ));
}

#[test]
fn test_strict_mode_strings() {
    assert!(!umt_is_value_nan_str_strict("NaN"));
    assert!(!umt_is_value_nan_str_strict("0"));
    assert!(!umt_is_value_nan_str_strict("a"));
    assert!(!umt_is_value_nan_str_strict(""));
}

#[test]
fn test_strict_mode_valid_numbers() {
    assert!(!umt_is_value_nan(0.0));
    assert!(!umt_is_value_nan(1.0));
    assert!(!umt_is_value_nan(-1.0));
    assert!(!umt_is_value_nan(f64::INFINITY));
}
