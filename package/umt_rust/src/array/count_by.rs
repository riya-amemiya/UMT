use std::collections::HashMap;
use std::hash::Hash;

/// Counts elements of an array grouped by an iteratee key.
///
/// # Arguments
///
/// * `array` - Array to count
/// * `iteratee` - Function to determine the group key for each element
///
/// # Returns
///
/// HashMap mapping keys to counts
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_count_by;
///
/// let arr = vec![6.1, 4.2, 6.3];
/// let result = umt_count_by(&arr, |x| (*x as i32));
/// assert_eq!(result.get(&4), Some(&1));
/// assert_eq!(result.get(&6), Some(&2));
/// ```
pub fn umt_count_by<T, K, F>(array: &[T], iteratee: F) -> HashMap<K, usize>
where
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut result: HashMap<K, usize> = HashMap::new();

    for item in array {
        *result.entry(iteratee(item)).or_insert(0) += 1;
    }

    result
}

/// Counts elements of an array with index access in the iteratee.
///
/// # Arguments
///
/// * `array` - Array to count
/// * `iteratee` - Function to determine the group key (receives value and index)
///
/// # Returns
///
/// HashMap mapping keys to counts
pub fn umt_count_by_indexed<T, K, F>(array: &[T], iteratee: F) -> HashMap<K, usize>
where
    K: Hash + Eq,
    F: Fn(&T, usize) -> K,
{
    let mut result: HashMap<K, usize> = HashMap::new();

    for (index, item) in array.iter().enumerate() {
        *result.entry(iteratee(item, index)).or_insert(0) += 1;
    }

    result
}
