use super::safe_execute::SafeResult;

/// Pattern-matches on a SafeResult, applying the appropriate handler
/// depending on whether the SafeResult is a success or an error.
///
/// # Arguments
/// * `result` - The SafeResult to match on
/// * `on_success` - Called with the value if the SafeResult is a success
/// * `on_error` - Called with the error if the SafeResult is an error
///
/// # Returns
/// The return value of whichever handler is invoked
///
/// # Example
/// ```
/// use umt_rust::error::{SafeResult, SuccessType, umt_match_result};
///
/// let result: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 42 });
/// let output = umt_match_result(result, |v| format!("Got {}", v), |e| format!("Failed: {}", e));
/// assert_eq!(output, "Got 42");
/// ```
#[inline]
pub fn umt_match_result<V, E, R, FS, FE>(
    result: SafeResult<V, E>,
    on_success: FS,
    on_error: FE,
) -> R
where
    FS: FnOnce(V) -> R,
    FE: FnOnce(E) -> R,
{
    match result {
        SafeResult::Success(s) => on_success(s.value),
        SafeResult::Error(e) => on_error(e.error),
    }
}
