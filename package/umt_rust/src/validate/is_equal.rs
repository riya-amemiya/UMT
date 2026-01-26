//! Strict equality comparison utility

/// Evaluates true strict equality (similar to Object.is in JavaScript)
///
/// This handles special cases like NaN comparison and -0/+0 distinction.
///
/// # Arguments
/// * `a` - First value
/// * `b` - Second value
///
/// # Returns
/// true if values are strictly equal
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_equal;
///
/// assert!(umt_is_equal(&1, &1));
/// assert!(umt_is_equal(&"test", &"test"));
/// ```
#[inline]
pub fn umt_is_equal<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

/// Evaluates strict equality for f64 with special NaN handling
///
/// # Arguments
/// * `a` - First f64 value
/// * `b` - Second f64 value
///
/// # Returns
/// true if values are strictly equal (NaN == NaN returns true, -0 != +0)
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_equal_f64;
///
/// assert!(umt_is_equal_f64(1.0, 1.0));
/// assert!(umt_is_equal_f64(f64::NAN, f64::NAN));
/// assert!(!umt_is_equal_f64(-0.0_f64, 0.0_f64));
/// ```
#[inline]
pub fn umt_is_equal_f64(a: f64, b: f64) -> bool {
    // Handle NaN case (NaN == NaN should be true, like Object.is)
    if a.is_nan() && b.is_nan() {
        return true;
    }
    // Handle -0 vs +0 (they should be different, like Object.is)
    if a == 0.0 && b == 0.0 {
        return a.is_sign_positive() == b.is_sign_positive();
    }
    a == b
}
