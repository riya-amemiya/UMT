use super::safe_execute::ErrorType;

/// Creates an error result.
///
/// Mirrors the TypeScript `errorFunction` which wraps an error value into an
/// `{ type: "error", error }` shape. In Rust the equivalent is the
/// [`ErrorType`] struct.
///
/// # Arguments
///
/// * `error` - The error value to wrap
///
/// # Returns
///
/// An [`ErrorType`] containing the error value
///
/// # Examples
///
/// ```
/// use umt_rust::error::umt_error_function;
///
/// let result = umt_error_function("something went wrong");
/// assert_eq!(result.error, "something went wrong");
/// ```
#[inline]
pub fn umt_error_function<E>(error: E) -> ErrorType<E> {
    ErrorType { error }
}
