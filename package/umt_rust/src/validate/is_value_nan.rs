//! NaN value checking functionality
//!
//! This module provides a function to check if a value is NaN.

/// Determines if a f64 value is NaN (strict mode)
///
/// Uses Number.isNaN semantics - only returns true for actual NaN values.
///
/// # Arguments
///
/// * `value` - The f64 value to check
///
/// # Returns
///
/// `true` if the value is NaN, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_value_nan;
///
/// assert!(umt_is_value_nan(f64::NAN));
/// assert!(!umt_is_value_nan(1.0));
/// assert!(!umt_is_value_nan(f64::INFINITY));
/// ```
#[inline]
pub fn umt_is_value_nan(value: f64) -> bool {
    value.is_nan()
}

/// Determines if a string represents NaN (loose mode)
///
/// Uses global isNaN semantics - returns true for values that would
/// coerce to NaN.
///
/// # Arguments
///
/// * `value` - The string to check
///
/// # Returns
///
/// `true` if the value is NaN or cannot be parsed as a number, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_value_nan_str_loose;
///
/// assert!(umt_is_value_nan_str_loose("NaN"));
/// assert!(umt_is_value_nan_str_loose("abc"));
/// assert!(!umt_is_value_nan_str_loose("0"));
/// assert!(!umt_is_value_nan_str_loose("1"));
/// ```
#[inline]
pub fn umt_is_value_nan_str_loose(value: &str) -> bool {
    // Empty string coerces to 0 in JavaScript (not NaN)
    if value.is_empty() || value.chars().all(|c| c.is_whitespace()) {
        return false;
    }
    match value.parse::<f64>() {
        Ok(num) => num.is_nan(),
        Err(_) => true,
    }
}

/// Determines if a string represents NaN (strict mode)
///
/// In strict mode, only actual NaN values return true.
/// Strings always return false in strict mode.
///
/// # Arguments
///
/// * `_value` - The string to check
///
/// # Returns
///
/// Always returns `false` since strings are not NaN in strict mode
#[inline]
pub fn umt_is_value_nan_str_strict(_value: &str) -> bool {
    false
}

/// Determines if a value is NaN with loose mode option
///
/// # Arguments
///
/// * `value` - The string value to check
/// * `loose` - If true, uses global isNaN semantics
///
/// # Returns
///
/// `true` if the value represents NaN, `false` otherwise
#[inline]
pub fn umt_is_value_nan_str(value: &str, loose: bool) -> bool {
    if loose {
        umt_is_value_nan_str_loose(value)
    } else {
        false
    }
}
