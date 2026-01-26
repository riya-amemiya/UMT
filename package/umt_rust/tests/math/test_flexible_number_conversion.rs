use umt_rust::math::{umt_flexible_number_conversion, umt_flexible_number_conversion_opt};

// Basic numeric input
#[test]
fn test_positive_integer_string() {
    assert_eq!(umt_flexible_number_conversion("123"), 123.0);
    assert_eq!(umt_flexible_number_conversion("456"), 456.0);
    assert_eq!(umt_flexible_number_conversion("789"), 789.0);
}

#[test]
fn test_negative_integer_string() {
    assert_eq!(umt_flexible_number_conversion("-123"), -123.0);
    assert_eq!(umt_flexible_number_conversion("-456"), -456.0);
    assert_eq!(umt_flexible_number_conversion("-789"), -789.0);
}

#[test]
fn test_decimal_string() {
    assert_eq!(umt_flexible_number_conversion("3.14"), 3.14);
    assert_eq!(umt_flexible_number_conversion("-2.718"), -2.718);
    assert_eq!(umt_flexible_number_conversion("0.001"), 0.001);
}

#[test]
fn test_exponential_notation() {
    assert_eq!(umt_flexible_number_conversion("1e3"), 1000.0);
    assert_eq!(umt_flexible_number_conversion("-2.5e-3"), -0.0025);
    assert_eq!(umt_flexible_number_conversion("6.022e23"), 6.022e23);
}

#[test]
fn test_special_base_notation() {
    assert_eq!(umt_flexible_number_conversion("0x1A"), 26.0);
    assert_eq!(umt_flexible_number_conversion("0b1011"), 11.0);
    assert_eq!(umt_flexible_number_conversion("0o77"), 63.0);
}

#[test]
fn test_string_with_spaces() {
    assert_eq!(umt_flexible_number_conversion(" 123 "), 123.0);
    assert_eq!(umt_flexible_number_conversion("   -456"), -456.0);
    assert_eq!(umt_flexible_number_conversion("789   "), 789.0);
}

#[test]
fn test_plus_sign() {
    assert!(umt_flexible_number_conversion("+").is_nan());
    assert_eq!(umt_flexible_number_conversion("+123"), 123.0);
    assert_eq!(umt_flexible_number_conversion("+0"), 0.0);
}

// Note: Rust handles invalid exponential notation differently from JavaScript
// The Rust implementation returns NaN for truncated exponential notations
#[test]
fn test_invalid_exponential() {
    // "1e" is parsed as invalid because 'e' needs a following number
    assert!(umt_flexible_number_conversion("1e").is_nan());
    assert!(umt_flexible_number_conversion("e10").is_nan());
    // "2e+" is parsed as invalid because 'e+' needs a following number
    assert!(umt_flexible_number_conversion("2e+").is_nan());
}

#[test]
fn test_decimal_point_only() {
    assert!(umt_flexible_number_conversion(".").is_nan());
    assert!(umt_flexible_number_conversion("-.").is_nan());
}

#[test]
fn test_non_numeric_prefix() {
    assert!(umt_flexible_number_conversion("px42").is_nan());
    assert!(umt_flexible_number_conversion("meter3.14").is_nan());
    assert!(umt_flexible_number_conversion("abc123").is_nan());
}

// Special numeric input
#[test]
fn test_infinity() {
    assert_eq!(umt_flexible_number_conversion("Infinity"), f64::INFINITY);
    assert_eq!(
        umt_flexible_number_conversion("-Infinity"),
        f64::NEG_INFINITY
    );
}

#[test]
fn test_nan_input() {
    assert!(umt_flexible_number_conversion("NaN").is_nan());
}

// Invalid input
#[test]
fn test_empty_string() {
    assert_eq!(umt_flexible_number_conversion(""), 0.0);
}

#[test]
fn test_invalid_strings() {
    assert!(umt_flexible_number_conversion("not a number").is_nan());
    assert!(umt_flexible_number_conversion("abc").is_nan());
}

#[test]
fn test_special_characters() {
    assert!(umt_flexible_number_conversion("@123").is_nan());
    assert!(umt_flexible_number_conversion("!@#$%").is_nan());
}

// Complex cases
#[test]
fn test_mixed_numbers_and_characters() {
    assert_eq!(umt_flexible_number_conversion("42px"), 42.0);
    assert_eq!(umt_flexible_number_conversion("-42px"), -42.0);
    assert_eq!(umt_flexible_number_conversion("3.14meters"), 3.14);
    assert_eq!(umt_flexible_number_conversion("-3.14meters"), -3.14);
    assert_eq!(umt_flexible_number_conversion("1e10meters"), 1e10);
}

#[test]
fn test_internal_spaces() {
    assert_eq!(umt_flexible_number_conversion(" 123 456 "), 123.0);
    assert_eq!(umt_flexible_number_conversion(" -456px "), -456.0);
    assert_eq!(umt_flexible_number_conversion("3.14 meters"), 3.14);
}

#[test]
fn test_leading_zeros() {
    assert_eq!(umt_flexible_number_conversion("0123"), 123.0);
    assert_eq!(umt_flexible_number_conversion("-0456"), -456.0);
    assert_eq!(umt_flexible_number_conversion("000789"), 789.0);
}

#[test]
fn test_leading_plus() {
    assert_eq!(umt_flexible_number_conversion("+123"), 123.0);
    assert_eq!(umt_flexible_number_conversion("+0"), 0.0);
    assert_eq!(umt_flexible_number_conversion("+3.14"), 3.14);
    assert_eq!(umt_flexible_number_conversion("+1e10"), 1e10);
}

// Option helper
#[test]
fn test_option_none() {
    assert_eq!(umt_flexible_number_conversion_opt(None), 0.0);
}

#[test]
fn test_option_some() {
    assert_eq!(umt_flexible_number_conversion_opt(Some("42")), 42.0);
}

use umt_rust::math::*;

#[test]
fn test_binary() {
    assert_eq!(umt_flexible_number_conversion("0b1010"), 10.0);
}

#[test]
fn test_float_string() {
    assert_eq!(umt_flexible_number_conversion("78.9"), 78.9);
}

#[test]
fn test_hex() {
    assert_eq!(umt_flexible_number_conversion("0xFF"), 255.0);
}

#[test]
fn test_integer_string() {
    assert_eq!(umt_flexible_number_conversion("123"), 123.0);
}

#[test]
fn test_not_a_number() {
    assert!(umt_flexible_number_conversion("not a number").is_nan());
}

#[test]
fn test_number_with_unit() {
    assert_eq!(umt_flexible_number_conversion("42px"), 42.0);
}

#[test]
fn test_octal() {
    assert_eq!(umt_flexible_number_conversion("0o10"), 8.0);
}

#[test]
fn test_scientific_notation() {
    assert_eq!(umt_flexible_number_conversion("3.14e2"), 314.0);
}
