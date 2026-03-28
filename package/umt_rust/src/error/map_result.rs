use super::safe_execute::{ErrorType, SafeResult, SuccessType};

/// Transforms the value inside a successful SafeResult using the provided mapping function.
/// If the SafeResult is an error, it is returned unchanged.
///
/// # Arguments
/// * `result` - The SafeResult to transform
/// * `f` - The function to apply to the success value
///
/// # Returns
/// A new SafeResult with the transformed value, or the original error
///
/// # Example
/// ```
/// use umt_rust::error::{SafeResult, SuccessType, ErrorType, umt_map_result};
///
/// let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
/// let result = umt_map_result(success, |n| n * 2);
/// assert_eq!(result.value(), Some(&10));
///
/// let error: SafeResult<i32, String> = SafeResult::Error(ErrorType { error: "fail".to_string() });
/// let result = umt_map_result(error, |n| n * 2);
/// assert!(result.is_error());
/// ```
#[inline]
pub fn umt_map_result<V, E, U, F>(result: SafeResult<V, E>, f: F) -> SafeResult<U, E>
where
    F: FnOnce(V) -> U,
{
    match result {
        SafeResult::Success(s) => SafeResult::Success(SuccessType { value: f(s.value) }),
        SafeResult::Error(e) => SafeResult::Error(e),
    }
}
