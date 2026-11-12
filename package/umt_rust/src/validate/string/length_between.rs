//! Inclusive string length range validation utility
//!
//! Provides a validator that checks whether a string's length lies between a
//! minimum and a maximum value, mirroring the TypeScript `lengthBetween()`
//! validator.

use super::super::core::ValidateReturnType;

/// Creates a validator for checking if a string's length is greater than or
/// equal to `min` and less than or equal to `max`.
///
/// The length is measured in UTF-16 code units, matching JavaScript's
/// `String.prototype.length`. Both bounds are inclusive and a single
/// `message` is reported for either bound.
///
/// # Arguments
/// * `min` - Minimum allowed length (inclusive)
/// * `max` - Maximum allowed length (inclusive)
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType<String>` validator for the inclusive length range
///
/// # Examples
/// ```
/// use umt_rust::validate::string::{umt_length_between, umt_validate_string};
///
/// let validator = umt_length_between(2, 4, Some("length must be 2 to 4".to_string()));
/// assert!(umt_validate_string("abc", &[validator.clone()], None).validate);
/// let result = umt_validate_string("abcde", &[validator], None);
/// assert_eq!(result.message, "length must be 2 to 4");
/// ```
#[inline]
pub fn umt_length_between(
    min: usize,
    max: usize,
    message: Option<String>,
) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| {
        let length = value.encode_utf16().count();
        length >= min && length <= max
    })
}
