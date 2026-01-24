//! Number validation module for double (floating point) numbers
//!
//! Provides validation functionality for checking if a number is a floating point value.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a number is a floating point value
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<f64>` validator for double numbers
///
/// # Examples
///
/// ```
/// use umt_rust::validate::number::umt_double;
///
/// let validator = umt_double(None);
/// assert!((validator.validate)(&1.5));
/// assert!((validator.validate)(&2.22));
/// assert!(!(validator.validate)(&1.0));
/// assert!(!(validator.validate)(&0.0));
/// ```
pub fn umt_double(message: Option<&str>) -> ValidateReturnType<f64> {
    ValidateReturnType::new(
        ValueType::Number,
        |value: &f64| value.is_finite() && value.fract() != 0.0,
        message.map(String::from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_true_for_double_numbers() {
        let validator = umt_double(None);
        assert!((validator.validate)(&1.5));
        assert!((validator.validate)(&2.22));
        assert!((validator.validate)(&0.1));
    }

    #[test]
    fn test_returns_false_for_non_double_numbers() {
        let validator = umt_double(None);
        assert!(!(validator.validate)(&1.0));
        assert!(!(validator.validate)(&100.0));
        assert!(!(validator.validate)(&0.0));
    }

    #[test]
    fn test_custom_message() {
        let custom_message = "This is not a double number";
        let validator = umt_double(Some(custom_message));
        assert!(!(validator.validate)(&1.0));
        assert_eq!(validator.message.as_deref(), Some(custom_message));
    }
}
