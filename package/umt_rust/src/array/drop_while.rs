/// Removes the leading elements of an array while the predicate returns true.
///
/// # Arguments
///
/// * `array` - The array to scan
/// * `predicate` - Function evaluated for each element until it first returns false
///
/// # Returns
///
/// A new vector starting at the first element that fails the predicate
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_drop_while;
///
/// assert_eq!(umt_drop_while(&[1, 2, 3, 4, 1], |n| *n < 3), vec![3, 4, 1]);
/// ```
pub fn umt_drop_while<T, F>(array: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    umt_drop_while_indexed(array, |item, _| predicate(item))
}

/// Removes the leading elements of an array while the predicate returns true,
/// with index access in the predicate.
///
/// # Arguments
///
/// * `array` - The array to scan
/// * `predicate` - Function evaluated for each element until it first returns false
///   (receives value and index)
///
/// # Returns
///
/// A new vector starting at the first element that fails the predicate
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_drop_while_indexed;
///
/// assert_eq!(umt_drop_while_indexed(&[5, 6, 7], |_, i| i < 2), vec![7]);
/// ```
pub fn umt_drop_while_indexed<T, F>(array: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, usize) -> bool,
{
    let start = array
        .iter()
        .enumerate()
        .position(|(index, item)| !predicate(item, index))
        .unwrap_or(array.len());
    array[start..].to_vec()
}
