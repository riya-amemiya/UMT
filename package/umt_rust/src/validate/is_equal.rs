//! Strict equality comparison functionality
//!
//! This module provides a function to check strict equality between two values,
//! similar to JavaScript's Object.is().

/// Evaluates strict equality using Object.is semantics
///
/// This function implements the same-value equality algorithm, which differs from
/// the standard == operator in two ways:
/// - NaN is considered equal to NaN
/// - -0 is not considered equal to +0
///
/// # Arguments
///
/// * `a` - First value to compare
/// * `b` - Second value to compare
///
/// # Returns
///
/// `true` if values are strictly equal, `false` otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_equal;
///
/// assert!(umt_is_equal(1, 1));
/// assert!(umt_is_equal("test", "test"));
/// assert!(!umt_is_equal(1, 2));
/// ```
#[inline]
pub fn umt_is_equal<T: PartialEq>(a: T, b: T) -> bool {
    a == b
}

/// Evaluates strict equality for f64 values using Object.is semantics
///
/// Special cases:
/// - NaN == NaN is true
/// - -0 != +0
///
/// # Arguments
///
/// * `a` - First f64 value to compare
/// * `b` - Second f64 value to compare
///
/// # Returns
///
/// `true` if values are strictly equal using Object.is semantics
///
/// # Examples
///
/// ```
/// use umt_rust::validate::umt_is_equal_f64;
///
/// assert!(umt_is_equal_f64(1.0, 1.0));
/// assert!(umt_is_equal_f64(f64::NAN, f64::NAN));
/// assert!(!umt_is_equal_f64(-0.0, 0.0));
/// ```
#[inline]
pub fn umt_is_equal_f64(a: f64, b: f64) -> bool {
    // Handle NaN comparison - NaN == NaN should be true
    if a.is_nan() && b.is_nan() {
        return true;
    }
    // Handle -0 vs +0 - they should not be equal
    if a == 0.0 && b == 0.0 {
        return a.is_sign_positive() == b.is_sign_positive();
    }
    a == b
}

/// Evaluates strict equality for f32 values using Object.is semantics
///
/// Special cases:
/// - NaN == NaN is true
/// - -0 != +0
///
/// # Arguments
///
/// * `a` - First f32 value to compare
/// * `b` - Second f32 value to compare
///
/// # Returns
///
/// `true` if values are strictly equal using Object.is semantics
#[inline]
pub fn umt_is_equal_f32(a: f32, b: f32) -> bool {
    // Handle NaN comparison - NaN == NaN should be true
    if a.is_nan() && b.is_nan() {
        return true;
    }
    // Handle -0 vs +0 - they should not be equal
    if a == 0.0 && b == 0.0 {
        return a.is_sign_positive() == b.is_sign_positive();
    }
    a == b
}

/// Checks if two references point to the same memory location
///
/// # Arguments
///
/// * `a` - First reference
/// * `b` - Second reference
///
/// # Returns
///
/// `true` if references point to the same memory location
#[inline]
pub fn umt_is_same_reference<T>(a: &T, b: &T) -> bool {
    std::ptr::eq(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_primitive_values() {
        assert!(umt_is_equal(1, 1));
        assert!(umt_is_equal("test", "test"));
        assert!(umt_is_equal(true, true));
    }

    #[test]
    fn test_different_primitive_values() {
        assert!(!umt_is_equal(1, 2));
        assert!(!umt_is_equal("test", "other"));
        assert!(!umt_is_equal(true, false));
    }

    #[test]
    fn test_nan_comparison() {
        assert!(umt_is_equal_f64(f64::NAN, f64::NAN));
        assert!(umt_is_equal_f32(f32::NAN, f32::NAN));
    }

    #[test]
    fn test_negative_zero_vs_positive_zero() {
        assert!(!umt_is_equal_f64(-0.0, 0.0));
        assert!(!umt_is_equal_f64(0.0, -0.0));
        assert!(umt_is_equal_f64(0.0, 0.0));
        assert!(umt_is_equal_f64(-0.0, -0.0));

        assert!(!umt_is_equal_f32(-0.0_f32, 0.0_f32));
        assert!(umt_is_equal_f32(0.0_f32, 0.0_f32));
    }

    #[test]
    fn test_object_references() {
        let obj = vec![1, 2, 3];
        let obj_ref1 = &obj;
        let obj_ref2 = &obj;

        assert!(umt_is_same_reference(obj_ref1, obj_ref2));

        let obj2 = vec![1, 2, 3];
        assert!(!umt_is_same_reference(&obj, &obj2));
    }

    #[test]
    fn test_float_comparisons() {
        assert!(umt_is_equal_f64(1.0, 1.0));
        assert!(umt_is_equal_f64(-1.5, -1.5));
        assert!(!umt_is_equal_f64(1.0, 2.0));
    }

    #[test]
    fn test_infinity() {
        assert!(umt_is_equal_f64(f64::INFINITY, f64::INFINITY));
        assert!(umt_is_equal_f64(f64::NEG_INFINITY, f64::NEG_INFINITY));
        assert!(!umt_is_equal_f64(f64::INFINITY, f64::NEG_INFINITY));
    }
}
