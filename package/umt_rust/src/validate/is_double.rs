//! Double (floating point) number checking utility

/// Determines if the value is a floating point number (has decimal part)
///
/// # Arguments
/// * `x` - The number to check
///
/// # Returns
/// true if the number has a fractional part, false otherwise
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_double;
///
/// assert!(umt_is_double(0.1));
/// assert!(umt_is_double(3.14));
/// assert!(!umt_is_double(5.0));
/// assert!(!umt_is_double(f64::NAN));
/// ```
#[inline]
pub fn umt_is_double(x: f64) -> bool {
    if !x.is_finite() {
        return false;
    }
    x.fract() != 0.0
}

/// Determines if a string represents a double value
///
/// # Arguments
/// * `s` - The string to check
///
/// # Returns
/// true if the string represents a floating point number with decimal part
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_double_str;
///
/// assert!(umt_is_double_str("0.1"));
/// assert!(umt_is_double_str("3.14"));
/// assert!(!umt_is_double_str("5"));
/// assert!(!umt_is_double_str("abc"));
/// ```
#[inline]
pub fn umt_is_double_str(s: &str) -> bool {
    match s.parse::<f64>() {
        Ok(n) => umt_is_double(n),
        Err(_) => false,
    }
}
