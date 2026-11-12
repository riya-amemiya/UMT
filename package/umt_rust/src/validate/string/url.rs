//! URL string validation utility
//!
//! Provides a validator that checks whether a string is an absolute URL,
//! mirroring the TypeScript `url()` validator.

use super::super::core::ValidateReturnType;
use crate::url::umt_is_absolute_url;

/// Creates a validator for checking if a string is an absolute URL.
///
/// A string passes when it starts with an RFC 3986 scheme followed by a
/// colon, as checked by [`umt_is_absolute_url`].
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A `ValidateReturnType<String>` validator for URL strings
///
/// # Examples
/// ```
/// use umt_rust::validate::string::{umt_url, umt_validate_string};
///
/// let validator = umt_url(Some("must be a URL".to_string()));
/// assert!(umt_validate_string("https://example.com", &[validator.clone()], None).validate);
/// let result = umt_validate_string("/path/to/page", &[validator], None);
/// assert_eq!(result.message, "must be a URL");
/// ```
#[inline]
pub fn umt_url(message: Option<String>) -> ValidateReturnType<String> {
    ValidateReturnType::new("string", message, |value: &String| {
        umt_is_absolute_url(value)
    })
}
