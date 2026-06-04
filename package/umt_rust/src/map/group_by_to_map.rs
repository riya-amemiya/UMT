/// Groups elements of a slice into insertion-ordered buckets based on a key
/// function.
///
/// Mirrors the TypeScript `groupByToMap`, which returns a `Map` rather than a
/// plain object. Insertion order of keys is preserved (the first time a key is
/// produced fixes its position), and every element is cloned into the bucket of
/// its key. Because the result keeps the order in which keys were first seen,
/// it is returned as a `Vec<(K, Vec<T>)>` instead of a `HashMap`, which would
/// lose ordering.
///
/// # Arguments
///
/// * `array` - The slice to group
/// * `iteratee` - A function receiving the value, its index, and the full
///   slice, returning the group key for that element
///
/// # Returns
///
/// A `Vec` of `(key, bucket)` pairs in the order keys were first encountered,
/// where each bucket holds the elements that produced that key, in order.
///
/// # Examples
///
/// ```
/// use umt_rust::map::umt_group_by_to_map;
///
/// let numbers: [f64; 3] = [6.1, 4.2, 6.3];
/// let result = umt_group_by_to_map(&numbers, |value, _, _| value.floor() as i64);
/// assert_eq!(
///     result,
///     vec![(6_i64, vec![6.1, 6.3]), (4_i64, vec![4.2])]
/// );
///
/// let words = vec!["one", "two", "three"];
/// let by_len = umt_group_by_to_map(&words, |value, _, _| value.len());
/// assert_eq!(
///     by_len,
///     vec![(3_usize, vec!["one", "two"]), (5_usize, vec!["three"])]
/// );
/// ```
pub fn umt_group_by_to_map<T, K, F>(array: &[T], mut iteratee: F) -> Vec<(K, Vec<T>)>
where
    T: Clone,
    K: PartialEq,
    F: FnMut(&T, usize, &[T]) -> K,
{
    let mut result: Vec<(K, Vec<T>)> = Vec::new();
    for index in 0..array.len() {
        let value = &array[index];
        let key = iteratee(value, index, array);
        if let Some(bucket) = result
            .iter_mut()
            .find(|(existing, _)| *existing == key)
            .map(|(_, bucket)| bucket)
        {
            bucket.push(value.clone());
        } else {
            result.push((key, vec![value.clone()]));
        }
    }
    result
}
