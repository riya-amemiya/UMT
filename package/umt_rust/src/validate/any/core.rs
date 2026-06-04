//! Any validation core module
//!
//! Provides a validator that accepts any value, useful when a position in a
//! schema needs to remain wide open while still participating in `object`,
//! `union`, `intersection`, and other compositional helpers.

/// Return type produced by an `any` validator.
///
/// Exposes the literal `"any"` tag through the `value_type` field so downstream
/// helpers can map it back to the `any` runtime type, mirroring the
/// `AnyReturnType` shape from the TypeScript source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnyReturnType {
    pub validate: bool,
    pub message: String,
    pub value_type: &'static str,
}

/// Creates a validator that accepts any value
///
/// # Returns
///
/// A validator function that always succeeds, returning an [`AnyReturnType`]
/// whose `value_type` is the literal `"any"`.
///
/// # Examples
///
/// ```
/// use umt_rust::validate::any::umt_any;
///
/// let validator = umt_any::<i32>();
/// let result = validator(42);
/// assert!(result.validate);
/// assert_eq!(result.value_type, "any");
/// ```
pub fn umt_any<T>() -> impl Fn(T) -> AnyReturnType {
    move |_value: T| AnyReturnType {
        validate: true,
        message: String::new(),
        value_type: "any",
    }
}
