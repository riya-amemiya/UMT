//! Unknown validation core module
//!
//! Provides a validator that accepts any value but exposes it as `unknown`
//! to keep callers honest about narrowing before use.

/// Return type produced by an `unknown` validator.
///
/// Exposes the literal `"unknown"` tag through the `value_type` field so
/// downstream helpers can map it back to the `unknown` runtime type,
/// mirroring the `UnknownReturnType` shape from the TypeScript source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownReturnType {
    pub validate: bool,
    pub message: String,
    pub value_type: &'static str,
}

/// Creates a validator that accepts any value but typed as unknown
///
/// # Returns
///
/// A validator function that always succeeds, returning an
/// [`UnknownReturnType`] whose `value_type` is the literal `"unknown"`.
///
/// # Examples
///
/// ```
/// use umt_rust::validate::unknown::umt_unknown;
///
/// let validator = umt_unknown::<i32>();
/// let result = validator(42);
/// assert!(result.validate);
/// assert_eq!(result.value_type, "unknown");
/// ```
pub fn umt_unknown<T>() -> impl Fn(T) -> UnknownReturnType {
    move |_value: T| UnknownReturnType {
        validate: true,
        message: String::new(),
        value_type: "unknown",
    }
}
