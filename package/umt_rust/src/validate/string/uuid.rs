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
