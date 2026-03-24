//! Predicate AND combinator with short-circuit evaluation

use super::BoxPredicate;

/// Creates a predicate that returns true only when all given
/// predicates return true, using short-circuit evaluation
///
/// # Arguments
/// * `predicates` - The predicates to combine
///
/// # Returns
/// A combined predicate that returns true only if all predicates pass
///
/// # Examples
/// ```
/// use umt_rust::predicate::umt_every;
///
/// let is_positive_even = umt_every(vec![
///     Box::new(|n: &i64| *n > 0) as Box<dyn Fn(&i64) -> bool>,
///     Box::new(|n: &i64| *n % 2 == 0),
/// ]);
/// assert!(is_positive_even(&4));
/// assert!(!is_positive_even(&-2));
/// assert!(!is_positive_even(&3));
/// ```
#[inline]
pub fn umt_every<T>(predicates: Vec<BoxPredicate<T>>) -> impl Fn(&T) -> bool {
    move |value: &T| {
        for predicate in &predicates {
            if !predicate(value) {
                return false;
            }
        }
        true
    }
}
