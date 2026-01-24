//! NaN value checking utility

/// Determines if a value is NaN
///
/// # Arguments
/// * `value` - The f64 value to check
///
/// # Returns
/// true if the value is NaN, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_value_nan;
///
/// assert!(umt_is_value_nan(f64::NAN));
/// assert!(!umt_is_value_nan(1.0));
/// assert!(!umt_is_value_nan(0.0));
/// ```
#[inline]
pub fn umt_is_value_nan(value: f64) -> bool {
    value.is_nan()
}

/// Determines if a string value represents NaN (loose check)
///
/// # Arguments
/// * `value` - The string to check
///
/// # Returns
/// true if the string cannot be parsed as a valid number, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_value_nan_str;
///
/// assert!(umt_is_value_nan_str("NaN"));
/// assert!(umt_is_value_nan_str("not a number"));
/// assert!(!umt_is_value_nan_str("123"));
/// assert!(!umt_is_value_nan_str("3.14"));
/// ```
#[inline]
pub fn umt_is_value_nan_str(value: &str) -> bool {
    value.parse::<f64>().is_err() || value.eq_ignore_ascii_case("nan")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_value_nan() {
        assert!(umt_is_value_nan(f64::NAN));
        assert!(!umt_is_value_nan(1.0));
        assert!(!umt_is_value_nan(0.0));
        assert!(!umt_is_value_nan(f64::INFINITY));
    }

    #[test]
    fn test_is_value_nan_str() {
        assert!(umt_is_value_nan_str("NaN"));
        assert!(umt_is_value_nan_str("nan"));
        assert!(umt_is_value_nan_str("not a number"));
        assert!(umt_is_value_nan_str("abc"));
        assert!(!umt_is_value_nan_str("123"));
        assert!(!umt_is_value_nan_str("3.14"));
        assert!(!umt_is_value_nan_str("-42"));
    }
}
