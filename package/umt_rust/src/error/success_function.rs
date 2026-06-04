use super::safe_execute::SuccessType;

/// Creates a success result.
///
/// Mirrors the TypeScript `successFunction` which wraps a value into an
/// `{ type: "success", value }` shape. In Rust the equivalent is the
/// [`SuccessType`] struct.
///
/// # Arguments
///
/// * `value` - The success value to wrap
///
/// # Returns
///
/// A [`SuccessType`] containing the value
///
/// # Examples
///
/// ```
/// use umt_rust::error::umt_success_function;
///
/// let result = umt_success_function(42);
/// assert_eq!(result.value, 42);
/// ```
#[inline]
pub fn umt_success_function<V>(value: V) -> SuccessType<V> {
    SuccessType { value }
}
