/// Combines two arrays by padding shorter arrays with None values
/// to match the length of the longest array.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector of tuples with Option-wrapped elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_zip_longest;
///
/// let a = vec![1, 2];
/// let b = vec!["a"];
/// assert_eq!(
///     umt_zip_longest(&a, &b),
///     vec![(Some(1), Some("a")), (Some(2), None)]
/// );
/// ```
pub fn umt_zip_longest<T: Clone, U: Clone>(
    array1: &[T],
    array2: &[U],
) -> Vec<(Option<T>, Option<U>)> {
    let max_length = array1.len().max(array2.len());
    (0..max_length)
        .map(|i| (array1.get(i).cloned(), array2.get(i).cloned()))
        .collect()
}

/// Combines three arrays by padding shorter arrays with None values.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
/// * `array3` - Third array
///
/// # Returns
///
/// Vector of tuples with Option-wrapped elements
pub fn umt_zip_longest3<T: Clone, U: Clone, V: Clone>(
    array1: &[T],
    array2: &[U],
    array3: &[V],
) -> Vec<(Option<T>, Option<U>, Option<V>)> {
    let max_length = array1.len().max(array2.len()).max(array3.len());
    (0..max_length)
        .map(|i| {
            (
                array1.get(i).cloned(),
                array2.get(i).cloned(),
                array3.get(i).cloned(),
            )
        })
        .collect()
}

/// Combines multiple arrays of the same type by padding shorter arrays with None values.
///
/// # Arguments
///
/// * `arrays` - Slice of arrays to combine
///
/// # Returns
///
/// Vector of vectors with Option-wrapped elements
pub fn umt_zip_longest_many<T: Clone>(arrays: &[&[T]]) -> Vec<Vec<Option<T>>> {
    if arrays.is_empty() {
        return vec![];
    }

    let max_length = arrays.iter().map(|a| a.len()).max().unwrap_or(0);
    (0..max_length)
        .map(|i| arrays.iter().map(|arr| arr.get(i).cloned()).collect())
        .collect()
}
