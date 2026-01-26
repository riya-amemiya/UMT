//! String validation module for maximum length check
//!
//! Provides validation functionality for checking if a string's length is less than or equal to a maximum value.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a string's length is less than or equal to a maximum value
///
/// # Arguments
///
/// * `max_len` - Maximum allowed length of the string
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for maximum string length
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::umt_max_length;
///
/// let validator = umt_max_length(3, None);
/// assert!((validator.validate)(&"abc".to_string()));
/// assert!((validator.validate)(&"ab".to_string()));
/// assert!(!(validator.validate)(&"abcd".to_string()));
/// ```
pub fn umt_max_length(max_len: usize, message: Option<&str>) -> ValidateReturnType<String> {
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| value.len() <= max_len,
        message.map(String::from),
    )
}
