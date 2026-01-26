//! String validation module for regular expression matching
//!
//! Provides validation functionality for checking if a string matches a regular expression pattern.

use crate::validate::types::{ValidateReturnType, ValueType};
use regex::Regex;

/// Creates a validator for checking if a string matches a regular expression pattern
///
/// # Arguments
///
/// * `pattern` - Regular expression pattern to match against
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for regular expression matching
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::umt_regex_match;
/// use regex::Regex;
///
/// let pattern = Regex::new(r"^[a-z]+$").unwrap();
/// let validator = umt_regex_match(pattern, None);
/// assert!((validator.validate)(&"abc".to_string()));
/// assert!(!(validator.validate)(&"ABC".to_string()));
/// ```
pub fn umt_regex_match(pattern: Regex, message: Option<&str>) -> ValidateReturnType<String> {
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| pattern.is_match(value),
        message.map(String::from),
    )
}

/// Creates a validator for checking if a string matches a regular expression pattern string
///
/// # Arguments
///
/// * `pattern_str` - Regular expression pattern string
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for regular expression matching
/// Returns a validator that always fails if the pattern is invalid
pub fn umt_regex_match_str(pattern_str: &str, message: Option<&str>) -> ValidateReturnType<String> {
    match Regex::new(pattern_str) {
        Ok(pattern) => ValidateReturnType::new(
            ValueType::String,
            move |value: &String| pattern.is_match(value),
            message.map(String::from),
        ),
        Err(_) => ValidateReturnType::new(
            ValueType::String,
            |_: &String| false,
            Some("Invalid regex pattern".to_string()),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_patterns() {
        let pattern = Regex::new(r"^[a-z]+$").unwrap();
        let validator = umt_regex_match(pattern, None);
        assert!((validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"ABC".to_string()));
        assert!(!(validator.validate)(&"123".to_string()));
        assert!(!(validator.validate)(&"abc123".to_string()));
    }

    #[test]
    fn test_non_matching_patterns() {
        let pattern = Regex::new(r"^[0-9]+$").unwrap();
        let validator = umt_regex_match(pattern, None);
        assert!((validator.validate)(&"123".to_string()));
        assert!(!(validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"123abc".to_string()));
        assert!(!(validator.validate)(&"".to_string()));
    }

    #[test]
    fn test_custom_message() {
        let pattern = Regex::new(r"^[A-Z]+$").unwrap();
        let message = "Only uppercase letters are allowed";
        let validator = umt_regex_match(pattern, Some(message));
        assert!((validator.validate)(&"ABC".to_string()));
        assert!(!(validator.validate)(&"abc".to_string()));
        assert_eq!(validator.message.as_deref(), Some(message));
    }

    #[test]
    fn test_regex_match_str() {
        let validator = umt_regex_match_str(r"^[a-z]+$", None);
        assert!((validator.validate)(&"abc".to_string()));
        assert!(!(validator.validate)(&"ABC".to_string()));
    }

    #[test]
    fn test_invalid_regex_pattern() {
        let validator = umt_regex_match_str(r"[invalid(", None);
        assert!(!(validator.validate)(&"anything".to_string()));
    }
}
