/// Creates an insertion-ordered map from two slices, pairing the first as keys
/// and the second as values.
///
/// Mirrors the TypeScript `zipToMap`. Pairs are formed at corresponding
/// positions and iteration stops at the shorter length. Duplicate keys keep the
/// latest value, while the key's original position is preserved (matching JS
/// `Map.set` semantics). Because insertion order is observable, the result is a
/// `Vec<(K, V)>` rather than a `HashMap`.
///
/// # Arguments
///
/// * `keys` - The slice of keys
/// * `values` - The slice of values
///
/// # Returns
///
/// A `Vec` of `(key, value)` pairs in insertion order, with duplicate keys
/// collapsed to their latest value.
///
/// # Examples
///
/// ```
/// use umt_rust::map::umt_zip_to_map;
///
/// let result = umt_zip_to_map(&["a", "b"], &[1, 2]);
/// assert_eq!(result, vec![("a", 1), ("b", 2)]);
///
/// // Iteration stops at the shorter length.
/// let shorter = umt_zip_to_map(&["a", "b", "c"], &[1, 2]);
/// assert_eq!(shorter, vec![("a", 1), ("b", 2)]);
///
/// // Duplicate keys keep the latest value at the original position.
/// let dup = umt_zip_to_map(&["a", "b", "a"], &[1, 2, 3]);
/// assert_eq!(dup, vec![("a", 3), ("b", 2)]);
/// ```
pub fn umt_zip_to_map<K, V>(keys: &[K], values: &[V]) -> Vec<(K, V)>
where
    K: Clone + PartialEq,
    V: Clone,
{
    let length = keys.len().min(values.len());
    let mut result: Vec<(K, V)> = Vec::new();
    for index in 0..length {
        let key = keys[index].clone();
        let value = values[index].clone();
        if let Some(entry) = result.iter_mut().find(|(existing, _)| *existing == key) {
            entry.1 = value;
        } else {
            result.push((key, value));
        }
    }
    result
}
