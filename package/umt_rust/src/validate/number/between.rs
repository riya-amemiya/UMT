//! Inclusive number range validation utility
//!
//! Provides a validator that checks whether a number lies between a minimum
//! and a maximum value, mirroring the TypeScript `between()` validator.

use super::super::core::ValidateReturnType;

/// Creates a validator for checking if a number is greater than or equal to
/// `min` and less than or equal to `max`.
///
/// Both bounds are inclusive and a single `message` is reported for either
/// bound. `NaN` never passes.
///
/// # Arguments
/// * `min` - Minimum allowed value (inclusive)
/// * `max` - Maximum allowed value (inclusive)
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType<f64>` validator for the inclusive range
///
/// # Examples
/// ```
/// use umt_rust::validate::number::umt_between;
///
/// let validator = umt_between(1.0, 10.0, None);
/// assert!((validator.validate)(&1.0));
/// assert!((validator.validate)(&10.0));
/// assert!(!(validator.validate)(&10.5));
/// ```
#[inline]
pub fn umt_between(min: f64, max: f64, message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, move |value: &f64| {
        *value >= min && *value <= max
    })
}
