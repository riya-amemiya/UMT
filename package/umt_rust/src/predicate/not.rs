//! Predicate negation combinator

/// Creates a predicate that negates the given predicate
///
/// # Arguments
/// * `f` - The predicate to negate
///
/// # Returns
/// A new predicate that returns the opposite of the original
///
/// # Examples
/// ```
/// use umt_rust::predicate::umt_not;
///
/// let is_even = |n: &i64| n % 2 == 0;
/// let is_odd = umt_not(is_even);
/// assert!(is_odd(&3));
/// assert!(!is_odd(&4));
/// ```
#[inline]
pub fn umt_not<T, F>(f: F) -> impl Fn(&T) -> bool
where
    F: Fn(&T) -> bool,
{
    move |value: &T| !f(value)
}
