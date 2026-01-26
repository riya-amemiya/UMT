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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_with_message() {
        let validator = umt_length(3, Some("Length must be 3"));
        assert!((validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"ab".to_string()));
        assert!(!(validator.validate)(&"abcd".to_string()));
        assert_eq!(validator.message.as_deref(), Some("Length must be 3"));
    }

    #[test]
    fn test_length_without_message() {
        let validator = umt_length(3, None);
        assert!((validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"abcd".to_string()));
        assert!(validator.message.is_none());
    }

    #[test]
    fn test_length_zero() {
        let validator = umt_length(0, None);
        assert!((validator.validate)(&"".to_string()));
        assert!(!(validator.validate)(&"a".to_string()));
    }
}
