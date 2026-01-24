//! Number type checking functionality
//!
//! This module provides functions to check if a value represents a number.

/// Determines if a value is a finite number
///
/// # Arguments
///
/// * `value` - The f64 value to check
///
/// # Returns
///
/// `true` if the value is a finite number (not NaN or infinite), `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_number;
///
/// assert!(umt_is_number(5.0));
/// assert!(umt_is_number(-5.5));
/// assert!(!umt_is_number(f64::NAN));
/// assert!(!umt_is_number(f64::INFINITY));
/// ```
#[inline]
pub fn umt_is_number(value: f64) -> bool {
    value.is_finite()
}

/// Determines if a value is a finite integer
///
/// # Arguments
///
/// * `value` - The i64 value to check
///
/// # Returns
///
/// Always returns `true` since i64 is always a valid finite integer
#[inline]
pub fn umt_is_number_i64(_value: i64) -> bool {
    true
}

/// Determines if a string represents a valid number
///
/// In loose mode, this accepts string representations of numbers.
///
/// # Arguments
///
/// * `value` - The string to check
///
/// # Returns
///
/// `true` if the string can be parsed as a finite number, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_number_str;
///
/// assert!(umt_is_number_str("5"));
/// assert!(umt_is_number_str("-5.5"));
/// assert!(umt_is_number_str("0"));
/// assert!(!umt_is_number_str("abc"));
/// assert!(!umt_is_number_str("5abc"));
/// ```
#[inline]
pub fn umt_is_number_str(value: &str) -> bool {
    // Empty string is treated as 0 in JavaScript's isFinite
    if value.is_empty() {
        return true;
    }
    match value.parse::<f64>() {
        Ok(num) => num.is_finite(),
        Err(_) => false,
    }
}

/// Determines if a string represents a valid number (strict mode)
///
/// In strict mode, only actual numeric types are accepted.
///
/// # Arguments
///
/// * `value` - The string to check
///
/// # Returns
///
/// Always returns `false` since strings are not numbers in strict mode
#[inline]
pub fn umt_is_number_str_strict(_value: &str) -> bool {
    false
}

/// Determines if a value is a valid number with loose mode option
///
/// # Arguments
///
/// * `value` - The string value to check
/// * `loose` - If true, accepts string representations of numbers
///
/// # Returns
///
/// `true` if the value represents a valid number, `false` otherwise
#[inline]
pub fn umt_is_number_str_loose(value: &str, loose: bool) -> bool {
    if loose {
        umt_is_number_str(value)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_numbers() {
        assert!(umt_is_number(5.0));
        assert!(umt_is_number(-5.0));
        assert!(umt_is_number(0.0));
    }

    #[test]
    fn test_decimal_numbers() {
        assert!(umt_is_number(5.5));
        assert!(umt_is_number(-5.5));
        assert!(umt_is_number(0.0));
    }

    #[test]
    fn test_string_numbers_loose() {
        assert!(umt_is_number_str("5"));
        assert!(umt_is_number_str("-5.5"));
        assert!(umt_is_number_str("0"));
    }

    #[test]
    fn test_invalid_string_numbers() {
        assert!(!umt_is_number_str("abc"));
        assert!(!umt_is_number_str("5abc"));
    }

    #[test]
    fn test_string_numbers_strict() {
        assert!(!umt_is_number_str_strict("5"));
        assert!(!umt_is_number_str_strict("-5.5"));
    }

    #[test]
    fn test_empty_string() {
        // Due to JavaScript's isFinite("") returning true
        assert!(umt_is_number_str(""));
    }

    #[test]
    fn test_special_values() {
        assert!(!umt_is_number(f64::NAN));
        assert!(!umt_is_number(f64::INFINITY));
        assert!(!umt_is_number(f64::NEG_INFINITY));
    }

    #[test]
    fn test_i64_always_valid() {
        assert!(umt_is_number_i64(0));
        assert!(umt_is_number_i64(i64::MAX));
        assert!(umt_is_number_i64(i64::MIN));
    }

    #[test]
    fn test_loose_mode() {
        assert!(umt_is_number_str_loose("5", true));
        assert!(umt_is_number_str_loose("-5.5", true));
        assert!(!umt_is_number_str_loose("5", false));
        assert!(!umt_is_number_str_loose("abc", true));
    }
}
