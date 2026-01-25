//! String validation module for UUID strings
//!
//! Provides validation functionality for checking if a string is a valid UUID.

use crate::validate::types::{ValidateReturnType, ValueType};
use regex::Regex;

/// Creates a validator for checking if a string is a valid UUID
///
/// # Arguments
///
/// * `versions` - Array of supported UUID versions (default: [4])
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A `ValidateReturnType<String>` validator for UUID strings
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::umt_uuid;
///
/// let validator = umt_uuid(None, None);
/// assert!((validator.validate)(&"123e4567-e89b-42d3-a456-426614174000".to_string()));
/// assert!(!(validator.validate)(&"not-a-uuid".to_string()));
/// ```
pub fn umt_uuid(versions: Option<Vec<u8>>, message: Option<&str>) -> ValidateReturnType<String> {
    let versions = versions.unwrap_or_else(|| vec![4]);
    ValidateReturnType::new(
        ValueType::String,
        move |value: &String| {
            versions.iter().any(|version| {
                // Regular expression for specific UUID version
                let pattern = format!(
                    r"(?i)^[\da-f]{{8}}-?[\da-f]{{4}}-?{}[\da-f]{{3}}-?[89ab][\da-f]{{3}}-?[\da-f]{{12}}$",
                    version
                );
                Regex::new(&pattern)
                    .map(|re| re.is_match(value))
                    .unwrap_or(false)
            })
        },
        message.map(String::from),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_uuids() {
        let validator = umt_uuid(None, None);
        assert!((validator.validate)(
            &"123e4567-e89b-42d3-a456-426614174000".to_string()
        ));
        assert!((validator.validate)(
            &"3A86B1AB-237E-473D-A38F-06AA6A2A4783".to_string()
        ));
        assert!((validator.validate)(
            &"B86D03E7-7B2D-4710-897B-77D05A1F0B4B".to_string()
        ));
        assert!((validator.validate)(
            &"573F42D5-1734-45E1-9D4D-AA43E6DF697E".to_string()
        ));
    }

    #[test]
    fn test_valid_uuid_without_dashes() {
        let validator = umt_uuid(None, None);
        assert!((validator.validate)(
            &"A95E3F622D0E425795D3B239AF08115D".to_string()
        ));
    }

    #[test]
    fn test_invalid_uuids() {
        let validator = umt_uuid(None, None);
        assert!(!(validator.validate)(
            &"123e4567-e89b-42d3-a456-42661417400Z".to_string()
        ));
        assert!(!(validator.validate)(
            &"123e4567-e89b-92d3-a456-426614174000".to_string()
        )); // Invalid version (9)
    }

    #[test]
    fn test_custom_message() {
        let custom_message = "Invalid UUID format";
        let validator = umt_uuid(Some(vec![4]), Some(custom_message));
        let result = (validator.validate)(&"invalid-uuid".to_string());
        assert!(!result);
        assert_eq!(validator.message.as_deref(), Some(custom_message));
    }

    #[test]
    fn test_mixed_case_letters() {
        let validator = umt_uuid(None, None);
        assert!((validator.validate)(
            &"123e4567-E89b-42D3-a456-426614174000".to_string()
        ));
    }

    #[test]
    fn test_different_uuid_versions() {
        let validator = umt_uuid(Some(vec![1, 2, 3, 4, 5]), None);
        assert!((validator.validate)(
            &"123e4567-e89b-12d3-a456-426614174000".to_string()
        )); // v1
        assert!((validator.validate)(
            &"123e4567-e89b-22d3-a456-426614174000".to_string()
        )); // v2
        assert!((validator.validate)(
            &"123e4567-e89b-32d3-a456-426614174000".to_string()
        )); // v3
        assert!((validator.validate)(
            &"123e4567-e89b-42d3-a456-426614174000".to_string()
        )); // v4
        assert!((validator.validate)(
            &"123e4567-e89b-52d3-a456-426614174000".to_string()
        )); // v5
    }

    #[test]
    fn test_completely_invalid_format() {
        let validator = umt_uuid(None, None);
        assert!(!(validator.validate)(&"completely-invalid-format".to_string()));
    }

    #[test]
    fn test_empty_string() {
        let validator = umt_uuid(None, None);
        assert!(!(validator.validate)(&"".to_string()));
    }

    #[test]
    fn test_insufficient_length() {
        let validator = umt_uuid(None, None);
        assert!(!(validator.validate)(
            &"123e4567-e89b-42d3-a456-42661417400".to_string()
        ));
    }

    #[test]
    fn test_excessive_length() {
        let validator = umt_uuid(None, None);
        assert!(!(validator.validate)(
            &"123e4567-e89b-42d3-a456-4266141740001234".to_string()
        ));
    }
}
