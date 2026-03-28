use super::safe_execute::{ErrorType, SafeResult};

/// Transforms the value inside a successful SafeResult using a function
/// that itself returns a SafeResult. If the original SafeResult is an error,
/// it is returned unchanged.
///
/// # Arguments
/// * `result` - The SafeResult to transform
/// * `f` - The function to apply to the success value, which returns a new SafeResult
///
/// # Returns
/// The SafeResult returned by the mapping function, or the original error
///
/// # Example
/// ```
/// use umt_rust::error::{SafeResult, SuccessType, ErrorType, umt_flat_map_result};
///
/// let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
/// let result = umt_flat_map_result(success, |n| {
///     if n > 0 {
///         SafeResult::Success(SuccessType { value: n * 2 })
///     } else {
///         SafeResult::Error(ErrorType { error: "negative".to_string() })
///     }
/// });
/// assert_eq!(result.value(), Some(&10));
/// ```
#[inline]
pub fn umt_flat_map_result<V, E, U, F, G>(result: SafeResult<V, E>, f: F) -> SafeResult<U, G>
where
    F: FnOnce(V) -> SafeResult<U, G>,
    E: Into<G>,
{
    match result {
        SafeResult::Success(s) => f(s.value),
        SafeResult::Error(e) => SafeResult::Error(ErrorType {
            error: e.error.into(),
        }),
    }
}
