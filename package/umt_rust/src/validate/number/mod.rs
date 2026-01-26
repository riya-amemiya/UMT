//! Number validation module
//! Provides validation functionality for number values with various constraints

use super::core::{ValidateCoreReturnType, ValidateReturnType};
use super::{umt_is_double, umt_is_prime_number};

/// Validates a number value with optional validation rules
///
/// # Arguments
/// * `value` - The number to validate
/// * `options` - Array of validation rules to apply
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::number::umt_validate_number;
///
/// let result = umt_validate_number(42.0, &[], None);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_number(
    value: f64,
    options: &[ValidateReturnType<f64>],
    message: Option<&str>,
) -> ValidateCoreReturnType<f64> {
    if !value.is_finite() {
        return ValidateCoreReturnType {
            validate: false,
            message: message.unwrap_or("Value is not a valid number").to_string(),
            type_value: value,
        };
    }

    for option in options {
        if !(option.validate)(&value) {
            return ValidateCoreReturnType {
                validate: false,
                message: option.message.clone().unwrap_or_default(),
                type_value: value,
            };
        }
    }

    ValidateCoreReturnType {
        validate: true,
        message: String::new(),
        type_value: value,
    }
}

/// Creates a validator for checking if a number is a floating point value
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for double numbers
#[inline]
pub fn umt_double(message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, |value: &f64| umt_is_double(*value))
}

/// Creates a validator for checking if a number is even
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for even numbers
#[inline]
pub fn umt_even(message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, |value: &f64| {
        if umt_is_double(*value) {
            return false;
        }
        (*value as i64) % 2 == 0
    })
}

/// Creates a validator for checking if a number is odd
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for odd numbers
#[inline]
pub fn umt_odd(message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, |value: &f64| {
        if umt_is_double(*value) {
            return false;
        }
        (*value as i64) % 2 != 0
    })
}

/// Creates a validator for checking if a number is prime
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for prime numbers
#[inline]
pub fn umt_prime(message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, |value: &f64| {
        umt_is_prime_number(*value as i64)
    })
}

/// Creates a validator for checking if a number is >= minimum value
///
/// # Arguments
/// * `min` - Minimum allowed value
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for minimum value check
#[inline]
pub fn umt_min_value(min: f64, message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, move |value: &f64| *value >= min)
}

/// Creates a validator for checking if a number is <= maximum value
///
/// # Arguments
/// * `max` - Maximum allowed value
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for maximum value check
#[inline]
pub fn umt_max_value(max: f64, message: Option<String>) -> ValidateReturnType<f64> {
    ValidateReturnType::new("number", message, move |value: &f64| *value <= max)
}
