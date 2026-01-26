/// Creates a new vector by combining elements from two arrays at corresponding positions.
/// The length of the result is the minimum of the two input arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
///
/// # Returns
///
/// Vector of tuples with combined elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_zip;
///
/// let a = vec![1, 2, 3];
/// let b = vec!["a", "b", "c"];
/// assert_eq!(umt_zip(&a, &b), vec![(1, "a"), (2, "b"), (3, "c")]);
/// ```
pub fn umt_zip<T: Clone, U: Clone>(array1: &[T], array2: &[U]) -> Vec<(T, U)> {
    let length = array1.len().min(array2.len());
    (0..length)
        .map(|i| (array1[i].clone(), array2[i].clone()))
        .collect()
}

/// Creates a new vector by combining elements from three arrays at corresponding positions.
/// The length of the result is the minimum of all input arrays.
///
/// # Arguments
///
/// * `array1` - First array
/// * `array2` - Second array
/// * `array3` - Third array
///
/// # Returns
///
/// Vector of tuples with combined elements
pub fn umt_zip3<T: Clone, U: Clone, V: Clone>(
    array1: &[T],
    array2: &[U],
    array3: &[V],
) -> Vec<(T, U, V)> {
    let length = array1.len().min(array2.len()).min(array3.len());
    (0..length)
        .map(|i| (array1[i].clone(), array2[i].clone(), array3[i].clone()))
        .collect()
}

/// Creates a new vector by combining elements from multiple arrays of the same type.
/// The length of the result is the minimum of all input arrays.
///
/// # Arguments
///
/// * `arrays` - Slice of arrays to combine
///
/// # Returns
///
/// Vector of vectors with combined elements
pub fn umt_zip_many<T: Clone>(arrays: &[&[T]]) -> Vec<Vec<T>> {
    if arrays.is_empty() {
        return vec![];
    }

    let length = arrays.iter().map(|a| a.len()).min().unwrap_or(0);
    (0..length)
        .map(|i| arrays.iter().map(|arr| arr[i].clone()).collect())
        .collect()
}
