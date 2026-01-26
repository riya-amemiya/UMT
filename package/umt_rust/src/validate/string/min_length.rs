//! String validation module for minimum length check
//!
//! Provides validation functionality for checking if a string's length is greater than or equal to a minimum value.

use crate::validate::types::{ValidateReturnType, ValueType};

/// Creates a validator for checking if a string's length is greater than or equal to a minimum value
///
/// # Arguments
///
/// * `min_len` - Minimum allowed length of the string
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for minimum string length
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::umt_min_length;
///
/// let validator = umt_min_length(3, None);
/// assert!((validator.validate)(&"abc".to_string()));
/// assert!((validator.validate)(&"abcd".to_string()));
/// assert!(!(validator.validate)(&"ab".to_string()));
/// ```
pub fn umt_min_length(min_len: usize, message: Option<&str>) -> ValidateReturnType<String> {
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| value.len() >= min_len,
        message.map(String::from),
    )
}
