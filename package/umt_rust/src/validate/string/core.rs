//! String validation core module
//!
//! Provides the base validation functionality for string values.

use crate::validate::types::{ValidateCoreResult, ValidateReturnType, ValueType};

/// Creates a string validator with optional validation rules
///
/// # Arguments
///
/// * `rules` - Optional array of validation rules to apply
/// * `message` - Custom error message for type validation
///
/// # Returns
///
/// A validator function that checks if the value is a string and applies validation rules
///
/// # Examples
///
/// ```
/// use umt_rust::validate::string::{umt_string_validator, umt_min_length, umt_max_length};
///
/// // Basic string validation
/// let validator = umt_string_validator(vec![], None);
/// let result = validator("test");
/// assert!(result.validate);
///
/// // With min/max length validation rules
/// let rules = vec![umt_min_length(3, None), umt_max_length(10, None)];
/// let validator = umt_string_validator(rules, None);
/// assert!(validator("hello").validate);
/// assert!(!validator("ab").validate);
/// ```
pub fn umt_string_validator(
    rules: Vec<ValidateReturnType<String>>,
    _message: Option<&str>,
) -> impl Fn(&str) -> ValidateCoreResult<String> {
    move |value: &str| {
        // Apply validation rules
        for rule in &rules {
            if !(rule.validate)(&value.to_string()) {
                return ValidateCoreResult {
                    validate: false,
                    message: rule.message.clone().unwrap_or_default(),
                    value_type: ValueType::String,
                    value: Some(value.to_string()),
                };
            }
        }

        ValidateCoreResult::success(value.to_string(), ValueType::String)
    }
}

/// Creates a simple string validator without additional rules
///
/// # Arguments
///
/// * `message` - Custom error message for validation failure
///
/// # Returns
///
/// A validator function
pub fn umt_string_validator_simple(
    _message: Option<&str>,
) -> impl Fn(&str) -> ValidateCoreResult<String> {
    move |value: &str| ValidateCoreResult::success(value.to_string(), ValueType::String)
}
