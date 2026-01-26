use std::collections::HashMap;
use std::hash::Hash;

/// Groups elements of an array based on a given iteratee function.
///
/// # Arguments
///
/// * `array` - Array to group
/// * `iteratee` - Function to determine the group key for each element
///
/// # Returns
///
/// HashMap with grouped elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_group_by;
///
/// let arr = vec![6.1, 4.2, 6.3];
/// let result = umt_group_by(&arr, |x| (*x as i32));
/// assert_eq!(result.get(&4), Some(&vec![4.2]));
/// assert_eq!(result.get(&6), Some(&vec![6.1, 6.3]));
/// ```
pub fn umt_group_by<T, K, F>(array: &[T], iteratee: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut result: HashMap<K, Vec<T>> = HashMap::new();

    for item in array {
        let key = iteratee(item);
        result.entry(key).or_default().push(item.clone());
    }

    result
}

/// Groups elements of an array with index access in the iteratee.
///
/// # Arguments
///
/// * `array` - Array to group
/// * `iteratee` - Function to determine the group key (receives value and index)
///
/// # Returns
///
/// HashMap with grouped elements
pub fn umt_group_by_indexed<T, K, F>(array: &[T], iteratee: F) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Hash + Eq,
    F: Fn(&T, usize) -> K,
{
    let mut result: HashMap<K, Vec<T>> = HashMap::new();

    for (index, item) in array.iter().enumerate() {
        let key = iteratee(item, index);
        result.entry(key).or_default().push(item.clone());
    }

    result
}
