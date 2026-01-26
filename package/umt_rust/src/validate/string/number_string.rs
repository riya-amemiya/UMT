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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_number_strings() {
        let validator = umt_number_string(None);
        assert!((validator.validate)(&"123".to_string()));
        assert!((validator.validate)(&"0.56".to_string()));
        assert!((validator.validate)(&"-42".to_string()));
        assert!((validator.validate)(&"3.14159".to_string()));
    }

    #[test]
    fn test_invalid_number_strings() {
        let validator = umt_number_string(None);
        assert!(!(validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"123abc".to_string()));
        assert!(!(validator.validate)(&"NaN".to_string()));
    }

    #[test]
    fn test_empty_string() {
        let validator = umt_number_string(None);
        // Empty string is considered valid (like JavaScript behavior)
        assert!((validator.validate)(&"".to_string()));
    }

    #[test]
    fn test_custom_message() {
        let custom_message = "This is not a valid number string";
        let validator = umt_number_string(Some(custom_message));
        assert!(!(validator.validate)(&"abc".to_string()));
        assert_eq!(validator.message.as_deref(), Some(custom_message));
    }
}
