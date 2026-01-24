//! String validation module
//! Provides validation functionality for string values with various constraints

use regex::Regex;

use super::core::{ValidateCoreReturnType, ValidateReturnType};
use super::{umt_is_number_str, umt_parse_email, ParseEmailLevel, ParseEmailOptions};

/// Validates a string value with optional validation rules
///
/// # Arguments
/// * `value` - The string to validate
/// * `options` - Array of validation rules to apply
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::string::umt_validate_string;
///
/// let result = umt_validate_string("hello", &[], None);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_string(
    value: &str,
    options: &[ValidateReturnType<String>],
    message: Option<&str>,
) -> ValidateCoreReturnType<String> {
    let value_string = value.to_string();

    for option in options {
        if !(option.validate)(&value_string) {
            return ValidateCoreReturnType {
                validate: false,
                message: option.message.clone().unwrap_or_default(),
                type_value: value_string,
            };
        }
    }

    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value_string,
    }
}

/// Creates a validator for checking if a string has an exact length
///
/// # Arguments
/// * `len` - The exact length the string should have
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for string length validation
#[inline]
pub fn umt_length(len: usize, message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| value.len() == len)
}

/// Creates a validator for checking if a string's length >= minimum
///
/// # Arguments
/// * `min` - Minimum allowed length
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for minimum string length validation
#[inline]
pub fn umt_min_length(min: usize, message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| value.len() >= min)
}

/// Creates a validator for checking if a string's length <= maximum
///
/// # Arguments
/// * `max` - Maximum allowed length
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for maximum string length validation
#[inline]
pub fn umt_max_length(max: usize, message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| value.len() <= max)
}

/// Creates a validator for checking if a string represents a valid number
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for number string validation
#[inline]
pub fn umt_number_string(message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, |value: &String| umt_is_number_str(value))
}

/// Creates a validator for checking if a string matches a regex pattern
///
/// # Arguments
/// * `pattern` - The regex pattern to match against
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for regex matching validation
#[inline]
pub fn umt_regex_match(pattern: Regex, message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| {
        pattern.is_match(value)
    })
}

/// Creates a validator for checking if a string is a valid UUID
///
/// # Arguments
/// * `versions` - Array of supported UUID versions (default: [4])
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for UUID string validation
#[inline]
pub fn umt_uuid(versions: Option<Vec<u8>>, message: Option<String>) -> ValidateReturnType<String> {
    let versions = versions.unwrap_or_else(|| vec![4]);
    ValidateReturnType::new("string", message, move |value: &String| {
        versions.iter().any(|version| {
            let pattern = format!(
                r"(?i)^[\da-f]{{8}}-?[\da-f]{{4}}-?{}[\da-f]{{3}}-?[89ab][\da-f]{{3}}-?[\da-f]{{12}}$",
                version
            );
            Regex::new(&pattern).map(|re| re.is_match(value)).unwrap_or(false)
        })
    })
}

/// Creates a validator for checking if a string is a valid email address
///
/// # Arguments
/// * `message` - Custom error message for validation failure
/// * `level` - Email validation level (default: Basic)
///
/// # Returns
/// A `ValidateReturnType` for email validation
#[inline]
pub fn umt_validate_email_validator(
    message: Option<String>,
    level: Option<ParseEmailLevel>,
) -> ValidateReturnType<String> {
    let email_level = level.unwrap_or(ParseEmailLevel::Basic);
    ValidateReturnType::new("string", message, move |value: &String| {
        umt_parse_email(value, Some(ParseEmailOptions { level: email_level })).valid
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_string() {
        let result = umt_validate_string("hello", &[], None);
        assert!(result.validate);
    }

    #[test]
    fn test_length() {
        let validator = umt_length(5, None);
        assert!((validator.validate)(&"hello".to_string()));
        assert!(!(validator.validate)(&"hi".to_string()));
    }

    #[test]
    fn test_min_length() {
        let validator = umt_min_length(3, None);
        assert!((validator.validate)(&"hello".to_string()));
        assert!((validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"ab".to_string()));
    }

    #[test]
    fn test_max_length() {
        let validator = umt_max_length(5, None);
        assert!((validator.validate)(&"hello".to_string()));
        assert!((validator.validate)(&"hi".to_string()));
        assert!(!(validator.validate)(&"hello world".to_string()));
    }

    #[test]
    fn test_number_string() {
        let validator = umt_number_string(None);
        assert!((validator.validate)(&"123".to_string()));
        assert!((validator.validate)(&"3.14".to_string()));
        assert!(!(validator.validate)(&"abc".to_string()));
    }

    #[test]
    fn test_regex_match() {
        let validator = umt_regex_match(Regex::new(r"^\d+$").unwrap(), None);
        assert!((validator.validate)(&"123".to_string()));
        assert!(!(validator.validate)(&"abc".to_string()));
    }

    #[test]
    fn test_uuid() {
        let validator = umt_uuid(None, None);
        assert!((validator.validate)(&"550e8400-e29b-41d4-a716-446655440000".to_string()));
        assert!(!(validator.validate)(&"not-a-uuid".to_string()));
    }

    #[test]
    fn test_validate_email() {
        let validator = umt_validate_email_validator(None, None);
        assert!((validator.validate)(&"user@example.com".to_string()));
        assert!(!(validator.validate)(&"invalid-email".to_string()));
    }
}
