//! String validation module for checking if a string represents a number
//!
//! Provides validation functionality for checking if a string can be converted to a number.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a string represents a valid number
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for number strings
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::umt_number_string;
///
/// let validator = umt_number_string(None);
/// assert!((validator.validate)(&"123".to_string()));
/// assert!((validator.validate)(&"0.56".to_string()));
/// assert!(!(validator.validate)(&"abc".to_string()));
/// ```
pub fn umt_number_string(message: Option<&str>) -> ValidateReturnType<String> {
    ValidateReturnType::new(
        ValueType::String,
        |value: &String| {
            // Empty string is considered valid (like JavaScript's isFinite(""))
            if value.is_empty() {
                return true;
            }
            match value.parse::<f64>() {
                Ok(num) => num.is_finite(),
                Err(_) => false,
            }
        },
        message.map(String::from),
    )
}
