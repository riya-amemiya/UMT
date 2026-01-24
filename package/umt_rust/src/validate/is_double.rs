//! Double (floating point) number checking functionality
//!
//! This module provides a function to check if a value is a decimal number.

/// Determines if a number is a decimal (floating point) number
///
/// A decimal number is a finite number that is not an integer.
///
/// # Arguments
///
/// * `value` - The f64 value to check
///
/// # Returns
///
/// `true` if the value is a decimal number (not an integer), `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_double;
///
/// assert!(umt_is_double(1.5));
/// assert!(umt_is_double(-1.5));
/// assert!(!umt_is_double(1.0));
/// assert!(!umt_is_double(f64::NAN));
/// assert!(!umt_is_double(f64::INFINITY));
/// ```
#[inline]
pub fn umt_is_double(value: f64) -> bool {
    value.is_finite() && value.fract() != 0.0
}

/// Determines if a string represents a decimal number
///
/// # Arguments
///
/// * `value` - The string to check
///
/// # Returns
///
/// `true` if the string represents a valid decimal number, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_double_str;
///
/// assert!(umt_is_double_str("1.5"));
/// assert!(umt_is_double_str("-1.5"));
/// assert!(!umt_is_double_str("1"));
/// assert!(!umt_is_double_str("abc"));
/// ```
#[inline]
pub fn umt_is_double_str(value: &str) -> bool {
    match value.parse::<f64>() {
        Ok(num) => umt_is_double(num),
        Err(_) => false,
    }
}

/// Determines if a value is a decimal number with loose mode option
///
/// In loose mode, the function accepts both f64 values and string representations.
///
/// # Arguments
///
/// * `value` - The value to check (can be f64 or &str)
/// * `loose` - If true, accepts string representations of numbers
///
/// # Returns
///
/// `true` if the value is a decimal number, `false` otherwise
pub enum NumberInput<'a> {
    Float(f64),
    Str(&'a str),
}

impl<'a> From<f64> for NumberInput<'a> {
    fn from(value: f64) -> Self {
        NumberInput::Float(value)
    }
}

impl<'a> From<&'a str> for NumberInput<'a> {
    fn from(value: &'a str) -> Self {
        NumberInput::Str(value)
    }
}

/// Determines if a value is a decimal number with loose mode support
///
/// # Arguments
///
/// * `value` - The value to check
/// * `loose` - If true, accepts string representations of numbers
///
/// # Returns
///
/// `true` if the value is a decimal number, `false` otherwise
#[inline]
pub fn umt_is_double_loose(value: NumberInput, loose: bool) -> bool {
    match value {
        NumberInput::Float(f) => umt_is_double(f),
        NumberInput::Str(s) => {
            if loose {
                umt_is_double_str(s)
            } else {
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_true_for_valid_doubles() {
        assert!(umt_is_double(1.5));
        assert!(umt_is_double(-1.5));
        assert!(umt_is_double(1.23e-4));
        assert!(umt_is_double(0.1));
        assert!(umt_is_double(2.22));
    }

    #[test]
    fn test_returns_false_for_integers() {
        assert!(!umt_is_double(1.0));
        assert!(!umt_is_double(-1.0));
        assert!(!umt_is_double(0.0));
        assert!(!umt_is_double(100.0));
    }

    #[test]
    fn test_returns_false_for_special_values() {
        assert!(!umt_is_double(f64::NAN));
        assert!(!umt_is_double(f64::INFINITY));
        assert!(!umt_is_double(f64::NEG_INFINITY));
    }

    #[test]
    fn test_string_validation() {
        assert!(umt_is_double_str("1.5"));
        assert!(umt_is_double_str("-1.5"));
        assert!(!umt_is_double_str("1"));
        assert!(!umt_is_double_str("1.0"));
        assert!(!umt_is_double_str("abc"));
        assert!(!umt_is_double_str(""));
    }

    #[test]
    fn test_loose_mode() {
        // Float input - loose doesn't matter for floats
        assert!(umt_is_double_loose(NumberInput::Float(1.5), true));
        assert!(umt_is_double_loose(NumberInput::Float(1.5), false));
        assert!(!umt_is_double_loose(NumberInput::Float(1.0), true));

        // String input
        assert!(umt_is_double_loose(NumberInput::Str("1.5"), true));
        assert!(!umt_is_double_loose(NumberInput::Str("1.5"), false));
        assert!(!umt_is_double_loose(NumberInput::Str("1"), true));
    }

    #[test]
    fn test_edge_cases() {
        // Very small decimals
        assert!(umt_is_double(0.000001));
        // Very large decimals
        assert!(umt_is_double(1000000.5));
        // Hexadecimal integers converted to float
        assert!(!umt_is_double(18.0)); // 0x12 = 18
    }
}
