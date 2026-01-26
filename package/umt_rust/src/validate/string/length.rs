//! String validation module for exact length check
//!
//! Provides validation functionality for checking if a string has an exact length.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a string has an exact length
///
/// # Arguments
///
/// * `len` - The exact length that the string should have
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for string length
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::umt_length;
///
/// let validator = umt_length(3, None);
/// assert!((validator.validate)(&"abc".to_string()));
/// assert!(!(validator.validate)(&"ab".to_string()));
/// assert!(!(validator.validate)(&"abcd".to_string()));
/// ```
pub fn umt_length(len: usize, message: Option<&str>) -> ValidateReturnType<String> {
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| value.len() == len,
        message.map(String::from),
    )
}
