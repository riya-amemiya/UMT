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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_true_for_even_numbers() {
        let validator = umt_even(None);
        assert!((validator.validate)(&2.0));
        assert!((validator.validate)(&4.0));
        assert!((validator.validate)(&6.0));
        assert!((validator.validate)(&8.0));
        assert!((validator.validate)(&10.0));
        assert!((validator.validate)(&100.0));
        assert!((validator.validate)(&1000.0));
    }

    #[test]
    fn test_returns_false_for_odd_numbers() {
        let validator = umt_even(None);
        assert!(!(validator.validate)(&1.0));
        assert!(!(validator.validate)(&3.0));
        assert!(!(validator.validate)(&5.0));
        assert!(!(validator.validate)(&7.0));
        assert!(!(validator.validate)(&9.0));
        assert!(!(validator.validate)(&99.0));
        assert!(!(validator.validate)(&999.0));
    }

    #[test]
    fn test_custom_message() {
        let custom_message = "This is a custom message";
        let validator = umt_even(Some(custom_message));
        assert!(!(validator.validate)(&1.0));
        assert_eq!(validator.message.as_deref(), Some(custom_message));
    }

    #[test]
    fn test_zero_is_even() {
        let validator = umt_even(None);
        assert!((validator.validate)(&0.0));
    }

    #[test]
    fn test_negative_numbers() {
        let validator = umt_even(None);
        assert!((validator.validate)(&-2.0));
        assert!((validator.validate)(&-4.0));
        assert!((validator.validate)(&-6.0));
        assert!(!(validator.validate)(&-1.0));
        assert!(!(validator.validate)(&-3.0));
        assert!(!(validator.validate)(&-5.0));
    }

    #[test]
    fn test_decimal_numbers() {
        let validator = umt_even(None);
        assert!(!(validator.validate)(&2.5));
        assert!(!(validator.validate)(&4.7));
        assert!(!(validator.validate)(&1.3));
        assert!(!(validator.validate)(&3.9));
    }

    #[test]
    fn test_large_numbers() {
        let validator = umt_even(None);
        assert!((validator.validate)(&1_000_000.0));
        assert!(!(validator.validate)(&999_999.0));
    }

    #[test]
    fn test_even_i64() {
        let validator = umt_even_i64(None);
        assert!((validator.validate)(&2));
        assert!((validator.validate)(&0));
        assert!((validator.validate)(&-4));
        assert!(!(validator.validate)(&1));
        assert!(!(validator.validate)(&-3));
    }
}
