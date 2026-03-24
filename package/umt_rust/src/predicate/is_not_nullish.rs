//! Non-nullish value checker

/// Checks whether a value is not nullish (is Some)
///
/// This is the Rust equivalent of JavaScript's `value !== null && value !== undefined`.
/// In Rust, the concept maps to `Option::is_some()`.
///
/// # Arguments
/// * `value` - The optional value to check
///
/// # Returns
/// `true` if the value is `Some`
///
/// # Examples
/// ```
/// use umt_rust::predicate::umt_is_not_nullish;
///
/// assert!(!umt_is_not_nullish(&None::<i32>));
/// assert!(umt_is_not_nullish(&Some(0)));
/// assert!(umt_is_not_nullish(&Some("")));
/// ```
#[inline]
pub fn umt_is_not_nullish<T>(value: &Option<T>) -> bool {
    value.is_some()
}
