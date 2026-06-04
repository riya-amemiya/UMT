//! Never validation utility
//!
//! Provides a validator that fails for every input, useful for marking
//! positions in a schema that must never be filled (for example, exhaustive
//! union members that should be unreachable at the type level).

use super::core::ValidateCoreReturnType;

/// Creates a validator that always fails
///
/// The returned closure accepts any input and always reports a failing
/// result. The `type_value` field carries the literal `"never"` tag so the
/// outcome can be mapped back to the `never` type by downstream helpers,
/// mirroring the TypeScript `never()` validator.
///
/// # Arguments
/// * `message` - Custom error message for validation failure
///
/// # Returns
/// A function that always returns a failing `ValidateCoreReturnType` whose
/// `type_value` is the literal string `"never"`
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_never;
///
/// let validator = umt_never(None);
/// let result = validator(0);
/// assert!(!result.validate);
/// assert_eq!(result.type_value, "never");
///
/// let validator = umt_never(Some("must never be set".to_string()));
/// assert_eq!(validator("anything").message, "must never be set");
/// ```
#[inline]
pub fn umt_never<T>(message: Option<String>) -> impl Fn(T) -> ValidateCoreReturnType<&'static str> {
    move |_value: T| ValidateCoreReturnType {
        validate: false,
        message: message.clone().unwrap_or_default(),
        type_value: "never",
    }
}
