//! BigInt validation module
//! Provides validation functionality for bigint values

use super::core::ValidateCoreReturnType;

/// Validates a bigint value
///
/// # Arguments
/// * `value` - The bigint to validate
/// * `message` - Custom error message
///
/// # Returns
/// A `ValidateCoreReturnType` containing the validation result
///
/// # Examples
/// ```
/// use umt_rust::validate::bigint::umt_validate_bigint;
///
/// let result = umt_validate_bigint(42i128, None);
/// assert!(result.validate);
/// ```
#[inline]
pub fn umt_validate_bigint(value: i128, message: Option<&str>) -> ValidateCoreReturnType<i128> {
    ValidateCoreReturnType {
        validate: true,
        message: message.unwrap_or("").to_string(),
        type_value: value,
    }
}

/// Creates a bigint validator function
///
/// # Arguments
/// * `message` - Custom error message for type validation
///
/// # Returns
/// A function that validates bigint values
pub fn umt_bigint_validator(
    message: Option<String>,
) -> Box<dyn Fn(i128) -> ValidateCoreReturnType<i128>> {
    Box::new(move |value: i128| ValidateCoreReturnType {
        validate: true,
        message: message.clone().unwrap_or_default(),
        type_value: value,
    })
}
