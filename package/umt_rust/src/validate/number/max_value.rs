//! Number validation module for maximum value check
//!
//! Provides validation functionality for checking if a number is less than or equal to a maximum value.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a number is less than or equal to a maximum value
///
/// # Arguments
///
/// * `max` - Maximum allowed value
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<f64>` validator for maximum value check
///
/// # Examples
///
/// ```
/// use umt_rust::validate::number::umt_max_value;
///
/// let validator = umt_max_value(10.0, None);
/// assert!((validator.validate)(&5.0));
/// assert!((validator.validate)(&10.0));
/// assert!(!(validator.validate)(&11.0));
/// ```
pub fn umt_max_value(max: f64, message: Option<&str>) -> ValidateReturnType<f64> {
    ValidateReturnType::new(
        ValueType::Number,
        move |value: &f64| *value <= max,
        message.map(String::from),
    )
}

/// Creates a validator for checking if an integer is less than or equal to a maximum value
///
/// # Arguments
///
/// * `max` - Maximum allowed value
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<i64>` validator for maximum value check
pub fn umt_max_value_i64(max: i64, message: Option<&str>) -> ValidateReturnType<i64> {
    ValidateReturnType::new(
        ValueType::Number,
        move |value: &i64| *value <= max,
        message.map(String::from),
    )
}
