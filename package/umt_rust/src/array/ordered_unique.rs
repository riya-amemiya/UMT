use std::collections::HashSet;
use std::hash::Hash;

/// Collects the first value of each distinct key, in encounter order.
///
/// Internal helper shared by `umt_unique`, `umt_arrays_join`, `umt_get_arrays_common`,
/// `umt_get_arrays_diff` and `umt_uniq_by`. The first value of each key is cloned
/// unchanged into the result.
///
/// # Arguments
///
/// * `items` - References to the values to scan, in order
/// * `key` - Maps each value to its identity key
///
/// # Returns
///
/// The first-seen value of each key
///
/// # Examples
///
/// ```
/// use umt_rust::array::ordered_unique::ordered_unique_by;
///
/// assert_eq!(ordered_unique_by(&[3, 1, 3, 2, 1], |x| *x), vec![3, 1, 2]);
/// ```
pub fn ordered_unique_by<'a, T, K, I, F>(items: I, mut key: F) -> Vec<T>
where
    T: Clone + 'a,
    K: Hash + Eq,
    I: IntoIterator<Item = &'a T>,
    F: FnMut(&'a T) -> K,
{
    let mut seen: HashSet<K> = HashSet::new();
    let mut result = Vec::new();

    for item in items {
        if seen.insert(key(item)) {
            result.push(item.clone());
        }
    }

    result
}
