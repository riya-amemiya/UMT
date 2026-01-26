//! Boolean validation module
//! Provides validation functionality for boolean values

use super::core::ValidateCoreReturnType;

/// Validates a boolean value
///
/// # Arguments
/// * `value` - The boolean to validate
/// * `message` - Custom error message
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::boolean::umt_validate_boolean;
///
/// let result = umt_validate_boolean(true, None);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_boolean(value: bool, message: Option<&str>) -> ValidateCoreReturnType<bool> {
    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value,
    }
}

/// Creates a boolean validator function
///
/// # Arguments
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A function that validates boolean values
pub fn umt_boolean_validator(
    message: Option<String>,
) -> Box<dyn Fn(bool) -> ValidateCoreReturnType<bool>> {
    Box::new(move |value: bool| ValidateCoreReturnType {
        validate: true,
        message: message.clone().unwrap_or_default(),
        type_value: value,
    })
}
