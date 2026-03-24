//! Nullish value checker

/// Checks whether a value is nullish (None)
///
/// This is the Rust equivalent of JavaScript's `value === null || value === undefined`.
/// In Rust, the concept maps to `Option::is_none()`.
///
/// # Arguments
/// * `value` - The optional value to check
///
/// # Returns
/// `true` if the value is `None`
///
/// # Examples
/// ```
/// use umt_rust::predicate::umt_is_nullish;
///
/// assert!(umt_is_nullish(&None::<i32>));
/// assert!(!umt_is_nullish(&Some(0)));
/// assert!(!umt_is_nullish(&Some("")));
/// ```
#[inline]
pub fn umt_is_nullish<T>(value: &Option<T>) -> bool {
    value.is_none()
}
