//! Exact length string validation utility
//!
//! Provides a validator that checks whether a string has an exact length,
//! mirroring the TypeScript `exactLength()` validator.

use super::super::core::ValidateReturnType;

/// Creates a validator for checking if a string has an exact length.
///
/// The returned validator reports whether the input string's length equals
/// `length`. On failure the configured `message` (or an empty string when none
/// is given) is surfaced by the enclosing string validator, mirroring the
/// TypeScript `exactLength()` validator whose `validate` returns
/// `value.length === length`.
///
/// # Arguments
/// * `length` - The exact length that the string should have
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType<String>` validator for exact string length
///
/// # Examples
/// ```
/// use umt_rust::validate::string::{umt_exact_length, umt_validate_string};
///
/// let validator = umt_exact_length(3, Some("Length must be 3".to_string()));
/// assert!(umt_validate_string("abc", &[validator.clone()], None).validate);
/// assert!(!umt_validate_string("ab", &[validator.clone()], None).validate);
/// assert!(!umt_validate_string("abcd", &[validator], None).validate);
/// ```
#[inline]
pub fn umt_exact_length(length: usize, message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, move |value: &String| {
        value.len() == length
    })
}
