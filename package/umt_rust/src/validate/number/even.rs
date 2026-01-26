//! Number validation module for even numbers
//!
//! Provides validation functionality for checking if a number is even.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a number is even
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<f64>` validator for even numbers
///
/// # Examples
///
/// ```
/// use umt_rust::validate::number::umt_even;
///
/// let validator = umt_even(None);
/// assert!((validator.validate)(&2.0));
/// assert!((validator.validate)(&4.0));
/// assert!((validator.validate)(&0.0));
/// assert!(!(validator.validate)(&1.0));
/// assert!(!(validator.validate)(&3.0));
/// ```
pub fn umt_even(message: Option<&str>) -> ValidateReturnType<f64> {
    ValidateReturnType::new(
        ValueType::Number,
        |value: &f64| {
            // Check if it's a decimal (not even)
            if value.fract() != 0.0 {
                return false;
            }
            // Check if it's even
            (*value as i64) % 2 == 0
        },
        message.map(String::from),
    )
}

/// Creates a validator for checking if an integer is even
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<i64>` validator for even integers
pub fn umt_even_i64(message: Option<&str>) -> ValidateReturnType<i64> {
    ValidateReturnType::new(
        ValueType::Number,
        |value: &i64| value % 2 == 0,
        message.map(String::from),
    )
}
