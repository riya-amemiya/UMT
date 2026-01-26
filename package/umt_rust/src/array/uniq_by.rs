use std::collections::HashSet;
use std::hash::Hash;

/// Removes duplicate values from an array based on a selector function.
///
/// # Arguments
///
/// * `array` - The array to process
/// * `selector` - Function that returns the value to compare for uniqueness
///
/// # Returns
///
/// A new vector with unique values based on the selector
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_uniq_by;
///
/// let arr = vec![1.1, 1.2, 2.1, 2.2, 3.1];
/// let result = umt_uniq_by(&arr, |x| (*x as i32));
/// assert_eq!(result, vec![1.1, 2.1, 3.1]);
/// ```
pub fn umt_uniq_by<T, K, F>(array: &[T], selector: F) -> Vec<T>
where
    T: Clone,
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in array {
        let key = selector(item);
        if seen.insert(key) {
            result.push(item.clone());
        }
    }

    result
}
