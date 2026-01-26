//! String validation module
//! Provides validation functionality for string values with various constraints

use super::core::{ValidateCoreReturnType, ValidateReturnType};
use super::{ParseEmailLevel, ParseEmailOptions, umt_is_number_str, umt_parse_email};

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

/// Creates a validator for checking if a string matches a given predicate
///
/// # Arguments
/// * `matcher` - A closure that returns true if the string matches
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType` for pattern matching validation
#[inline]
pub fn umt_regex_match<F: Fn(&str) -> bool + Send + Sync + 'static>(
    matcher: F, message: Option<String>
) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| {
        matcher(value)
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
        is_valid_uuid(value, &versions)
    })
}

fn is_valid_uuid(value: &str, versions: &[u8]) -> bool {
    // UUID can be with or without dashes
    let hex_only: String = value.chars().filter(|c| *c != '-').collect();
    if hex_only.len() != 32 {
        return false;
    }
    if !hex_only.chars().all(|c| c.is_ascii_hexdigit()) {
        return false;
    }
    // Check version (13th hex char)
    let version_char = hex_only.as_bytes()[12];
    let version_num = if version_char.is_ascii_digit() {
        version_char - b'0'
    } else {
        return false;
    };
    if !versions.contains(&version_num) {
        return false;
    }
    // Check variant (17th hex char should be 8, 9, a, or b)
    let variant_char = hex_only.as_bytes()[16].to_ascii_lowercase();
    matches!(variant_char, b'8' | b'9' | b'a' | b'b')
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
        let validator = umt_regex_match(|s: &str| s.chars().all(|c| c.is_ascii_digit()), None);
        assert!((validator.validate)(&"123".to_string()));
        assert!(!(validator.validate)(&"abc".to_string()));
    }

    #[test]
    fn test_uuid() {
        let validator = umt_uuid(None, None);
        assert!((validator.validate)(
            &"550e8400-e29b-41d4-a716-446655440000".to_string()
        ));
        assert!(!(validator.validate)(&"not-a-uuid".to_string()));
    }

    #[test]
    fn test_validate_email() {
        let validator = umt_validate_email_validator(None, None);
        assert!((validator.validate)(&"user@example.com".to_string()));
        assert!(!(validator.validate)(&"invalid-email".to_string()));
    }
}
