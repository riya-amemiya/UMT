//! Number type checking utility

use super::js_number::js_string_to_number;

/// Determines if the value is a finite number
///
/// # Arguments
/// * `number` - The f64 value to check
///
/// # Returns
/// true if the value is a finite number, false otherwise (NaN, infinity)
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_number;
///
/// assert!(umt_is_number(0.1));
/// assert!(umt_is_number(42.0));
/// assert!(!umt_is_number(f64::NAN));
/// assert!(!umt_is_number(f64::INFINITY));
/// ```
#[inline]
pub fn umt_is_number(number: f64) -> bool {
    number.is_finite()
}

/// Determines if a string represents a valid number
///
/// The string is converted with JavaScript `Number(string)` rules, so
/// surrounding whitespace is ignored, `0x` / `0o` / `0b` literals are
/// accepted, and an empty or whitespace-only string converts to `0`.
///
/// # Arguments
/// * `s` - The string to check
///
/// # Returns
/// true if the string represents a valid finite number, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_number_str;
///
/// assert!(umt_is_number_str("0.1"));
/// assert!(umt_is_number_str("42"));
/// assert!(umt_is_number_str("-123.45"));
/// assert!(umt_is_number_str(" 0x10 "));
/// assert!(umt_is_number_str(""));
/// assert!(!umt_is_number_str("abc"));
/// ```
#[inline]
pub fn umt_is_number_str(s: &str) -> bool {
    js_string_to_number(s).is_finite()
}

/// Checks if an i64 is a valid number (always true for integers)
///
/// # Arguments
/// * `_number` - The integer to check
///
/// # Returns
/// Always returns true for i64 values
#[inline]
pub fn umt_is_number_i64(_number: i64) -> bool {
    true
}
