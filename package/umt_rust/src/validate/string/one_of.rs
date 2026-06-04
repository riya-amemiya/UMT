//! One-of (literal union) string validation utility
//!
//! Provides a validator that checks whether a string value is one of a fixed
//! set of allowed literal values, useful for validating string literal union
//! types such as `'standard' | 'squat' | 'decanter' | 'round' | 'tall'`.

use std::collections::HashSet;

use super::super::core::ValidateCoreReturnType;

/// Creates a top-level validator that checks if a string value is one of the
/// given allowed literal values.
///
/// The returned closure accepts a string and reports whether it is contained in
/// `values`. The match is exact and case sensitive. On success the message is
/// empty; on failure the configured `message` (or an empty string when none is
/// given) is returned. The `type_value` field carries the input value back so
/// downstream helpers can preserve the literal it matched, mirroring the
/// TypeScript `oneOf()` validator whose `type` field exposes the literal union.
///
/// # Arguments
/// * `values` - The collection of allowed string values
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A function from `&str` to a `ValidateCoreReturnType<String>` whose
/// `type_value` is the supplied value
///
/// # Examples
/// ```
/// use umt_rust::validate::string::umt_one_of;
///
/// let validator = umt_one_of(&["standard", "squat", "flask"], None);
/// assert!(validator("standard").validate);
/// assert!(!validator("unknown").validate);
///
/// let validator = umt_one_of(&["a", "b"], Some("must be a or b".to_string()));
/// assert_eq!(validator("c").message, "must be a or b");
/// assert_eq!(validator("a").message, "");
/// ```
#[inline]
pub fn umt_one_of(
    values: &[&str],
    message: Option<String>,
) -> impl Fn(&str) -> ValidateCoreReturnType<String> {
    let allowed: HashSet<String> = values.iter().map(|value| value.to_string()).collect();
    move |value: &str| {
        let is_valid = allowed.contains(value);
        ValidateCoreReturnType {
            validate: is_valid,
            message: if is_valid {
                String::new()
            } else {
                message.clone().unwrap_or_default()
            },
            type_value: value.to_string(),
        }
    }
}
