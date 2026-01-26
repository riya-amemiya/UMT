//! Number validation module for minimum value check
//!
//! Provides validation functionality for checking if a number is greater than or equal to a minimum value.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a number is greater than or equal to a minimum value
///
/// # Arguments
///
/// * `min` - Minimum allowed value
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<f64>` validator for minimum value check
///
/// # Examples
///
/// ```
/// use umt_rust::validate::number::umt_min_value;
///
/// let validator = umt_min_value(3.0, None);
/// assert!((validator.validate)(&5.0));
/// assert!((validator.validate)(&3.0));
/// assert!(!(validator.validate)(&2.0));
/// ```
pub fn umt_min_value(min: f64, message: Option<&str>) -> ValidateReturnType<f64> {
    ValidateReturnType::new(
        ValueType::Number,
        move |value: &f64| *value >= min,
        message.map(String::from),
    )
}

/// Creates a validator for checking if an integer is greater than or equal to a minimum value
///
/// # Arguments
///
/// * `min` - Minimum allowed value
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<i64>` validator for minimum value check
pub fn umt_min_value_i64(min: i64, message: Option<&str>) -> ValidateReturnType<i64> {
    ValidateReturnType::new(
        ValueType::Number,
        move |value: &i64| *value >= min,
        message.map(String::from),
    )
}
