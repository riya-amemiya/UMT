/// Splits a slice of pairs into two vectors (the inverse of `umt_zip`).
///
/// # Arguments
///
/// * `array` - Pairs to split
///
/// # Returns
///
/// A tuple of the first and second elements of every pair
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_unzip;
///
/// assert_eq!(umt_unzip(&[(1, "a"), (2, "b")]), (vec![1, 2], vec!["a", "b"]));
/// ```
pub fn umt_unzip<T: Clone, U: Clone>(array: &[(T, U)]) -> (Vec<T>, Vec<U>) {
    array.iter().cloned().unzip()
}

/// Splits a slice of triples into three vectors (the inverse of `umt_zip3`).
///
/// # Arguments
///
/// * `array` - Triples to split
///
/// # Returns
///
/// A tuple of the first, second and third elements of every triple
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_unzip3;
///
/// assert_eq!(
///     umt_unzip3(&[(1, "a", true), (2, "b", false)]),
///     (vec![1, 2], vec!["a", "b"], vec![true, false])
/// );
/// ```
pub fn umt_unzip3<T: Clone, U: Clone, V: Clone>(array: &[(T, U, V)]) -> (Vec<T>, Vec<U>, Vec<V>) {
    let mut first = Vec::with_capacity(array.len());
    let mut second = Vec::with_capacity(array.len());
    let mut third = Vec::with_capacity(array.len());
    for (a, b, c) in array {
        first.push(a.clone());
        second.push(b.clone());
        third.push(c.clone());
    }
    (first, second, third)
}

/// Splits rows of the same type into one vector per position (the inverse of `umt_zip_many`).
///
/// The number of result vectors is the shortest row length, matching the
/// truncation of `umt_zip_many`.
///
/// # Arguments
///
/// * `array` - Rows to split
///
/// # Returns
///
/// One vector per row position
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_unzip_many;
///
/// let rows = vec![vec![1, 2, 3], vec![4, 5]];
/// assert_eq!(umt_unzip_many(&rows), vec![vec![1, 4], vec![2, 5]]);
/// ```
pub fn umt_unzip_many<T: Clone, R: AsRef<[T]>>(array: &[R]) -> Vec<Vec<T>> {
    let width = match array.iter().map(|row| row.as_ref().len()).min() {
        Some(width) => width,
        None => return vec![],
    };
    (0..width)
        .map(|position| {
            array
                .iter()
                .map(|row| row.as_ref()[position].clone())
                .collect()
        })
        .collect()
}
