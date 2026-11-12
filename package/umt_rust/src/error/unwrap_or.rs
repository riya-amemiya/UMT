use super::safe_execute::SafeResult;

/// Extracts the value from a successful SafeResult, or returns the provided
/// default value if the SafeResult is an error.
///
/// # Arguments
/// * `result` - The SafeResult to unwrap
/// * `default_value` - The value returned when the SafeResult is an error
///
/// # Returns
/// The success value, or the default value for an error SafeResult
///
/// # Example
/// ```
/// use umt_rust::error::{SafeResult, SuccessType, ErrorType, umt_unwrap_or};
///
/// let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
/// assert_eq!(umt_unwrap_or(success, 0), 5);
///
/// let error: SafeResult<i32, String> = SafeResult::Error(ErrorType { error: "fail".to_string() });
/// assert_eq!(umt_unwrap_or(error, 0), 0);
/// ```
#[inline]
pub fn umt_unwrap_or<V, E>(result: SafeResult<V, E>, default_value: V) -> V {
    match result {
        SafeResult::Success(s) => s.value,
        SafeResult::Error(_) => default_value,
    }
}
