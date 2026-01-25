//! Number type checking utility

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
/// assert!(!umt_is_number_str("abc"));
/// assert!(!umt_is_number_str(""));
/// ```
#[inline]
pub fn umt_is_number_str(s: &str) -> bool {
    match s.parse::<f64>() {
        Ok(n) => n.is_finite(),
        Err(_) => false,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number() {
        assert!(umt_is_number(0.1));
        assert!(umt_is_number(42.0));
        assert!(umt_is_number(-123.45));
        assert!(umt_is_number(0.0));
        assert!(!umt_is_number(f64::NAN));
        assert!(!umt_is_number(f64::INFINITY));
        assert!(!umt_is_number(f64::NEG_INFINITY));
    }

    #[test]
    fn test_is_number_str() {
        assert!(umt_is_number_str("0.1"));
        assert!(umt_is_number_str("42"));
        assert!(umt_is_number_str("-123.45"));
        assert!(!umt_is_number_str("abc"));
        assert!(!umt_is_number_str(""));
        assert!(!umt_is_number_str("NaN"));
    }

    #[test]
    fn test_is_number_i64() {
        assert!(umt_is_number_i64(42));
        assert!(umt_is_number_i64(-100));
        assert!(umt_is_number_i64(0));
    }
}
