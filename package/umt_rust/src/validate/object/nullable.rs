//! Nullable validation utility
//!
//! Provides a validator wrapper that accepts null (`None`) values, passing
//! them through as valid while delegating every present (`Some`) value to the
//! wrapped validator. Mirrors the TypeScript `nullable()` validator.

use super::super::core::ValidateCoreReturnType;

/// Wraps a validator to accept null values
///
/// The returned closure receives an `Option`. When the value is `None` it
/// reports a passing result whose `type_value` carries the literal `"null"`
/// tag, mirroring the TypeScript `NullReturn`. When the value is `Some` it
/// delegates to the wrapped validator, forwarding its result unchanged.
///
/// # Arguments
/// * `validator` - The inner validator applied to present (`Some`) values
///
/// # Returns
/// A function that validates optional-by-null values, yielding the wrapped
/// validator's `ValidateCoreReturnType` for `Some` and a passing
/// `"null"`-tagged result for `None`
///
/// # Examples
/// ```
/// use umt_rust::validate::object::umt_nullable;
/// use umt_rust::validate::core::ValidateCoreReturnType;
///
/// let validator = umt_nullable(|value: &i32| ValidateCoreReturnType {
///     validate: *value > 0,
///     message: String::new(),
///     type_value: "number",
/// });
///
/// let null_result = validator(&None);
/// assert!(null_result.validate);
/// assert_eq!(null_result.type_value, "null");
///
/// assert!(validator(&Some(5)).validate);
/// assert!(!validator(&Some(-1)).validate);
/// ```
#[inline]
pub fn umt_nullable<T, F>(
    validator: F,
) -> impl Fn(&Option<T>) -> ValidateCoreReturnType<&'static str>
where
    F: Fn(&T) -> ValidateCoreReturnType<&'static str>,
{
    move |value: &Option<T>| match value {
        None => ValidateCoreReturnType {
            validate: true,
            message: String::new(),
            type_value: "null",
        },
        Some(inner) => validator(inner),
    }
}
