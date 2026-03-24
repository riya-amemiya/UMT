//! Predicate OR combinator with short-circuit evaluation

use super::BoxPredicate;

/// Creates a predicate that returns true when at least one of
/// the given predicates returns true, using short-circuit evaluation
///
/// # Arguments
/// * `predicates` - The predicates to combine
///
/// # Returns
/// A combined predicate that returns true if any predicate passes
///
/// # Examples
/// ```
/// use umt_rust::predicate::umt_some;
///
/// let is_zero_or_negative = umt_some(vec![
///     Box::new(|n: &i64| *n == 0) as Box<dyn Fn(&i64) -> bool>,
///     Box::new(|n: &i64| *n < 0),
/// ]);
/// assert!(is_zero_or_negative(&0));
/// assert!(is_zero_or_negative(&-5));
/// assert!(!is_zero_or_negative(&5));
/// ```
#[inline]
pub fn umt_some<T>(predicates: Vec<BoxPredicate<T>>) -> impl Fn(&T) -> bool {
    move |value: &T| {
        for predicate in &predicates {
            if predicate(value) {
                return true;
            }
        }
        false
    }
}
