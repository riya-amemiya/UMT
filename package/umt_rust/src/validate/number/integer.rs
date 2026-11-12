//! Integer number validation utility
//!
//! Provides a validator that checks whether a number is an integer,
//! mirroring the TypeScript `integer()` validator.

use super::super::core::ValidateReturnType;

/// Creates a validator for checking if a number is an integer.
///
/// Mirrors JavaScript's `Number.isInteger`: integral values such as `1.0`
/// pass, while fractional values, `NaN`, and infinities fail.
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType<f64>` validator for integer numbers
///
/// # Examples
/// ```
/// use umt_rust::validate::number::{umt_integer, umt_validate_number};
///
/// let validator = umt_integer(Some("must be an integer".to_string()));
/// assert!((validator.validate)(&3.0));
/// assert!(!(validator.validate)(&3.5));
/// assert!(!(validator.validate)(&f64::INFINITY));
///
/// let result = umt_validate_number(3.5, &[validator], None);
/// assert_eq!(result.message, "must be an integer");
/// ```
#[inline]
pub fn umt_integer(message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, |value: &f64| {
        value.is_finite() && value.fract() == 0.0
    })
}
