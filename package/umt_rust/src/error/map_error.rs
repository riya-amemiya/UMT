use super::safe_execute::{ErrorType, SafeResult};

/// Transforms the error inside a failed SafeResult using the provided mapping function.
/// If the SafeResult is a success, it is returned unchanged.
///
/// # Arguments
/// * `result` - The SafeResult to transform
/// * `f` - The function to apply to the error
///
/// # Returns
/// A new SafeResult with the transformed error, or the original success
///
/// # Example
/// ```
/// use umt_rust::error::{SafeResult, SuccessType, ErrorType, umt_map_error};
///
/// let error: SafeResult<i32, String> = SafeResult::Error(ErrorType { error: "fail".to_string() });
/// let result = umt_map_error(error, |message| message.len());
/// assert_eq!(result.error(), Some(&4));
///
/// let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
/// let result = umt_map_error(success, |message| message.len());
/// assert_eq!(result.value(), Some(&5));
/// ```
#[inline]
pub fn umt_map_error<V, E, F, M>(result: SafeResult<V, E>, f: M) -> SafeResult<V, F>
where
    M: FnOnce(E) -> F,
{
    match result {
        SafeResult::Success(s) => SafeResult::Success(s),
        SafeResult::Error(e) => SafeResult::Error(ErrorType { error: f(e.error) }),
    }
}
