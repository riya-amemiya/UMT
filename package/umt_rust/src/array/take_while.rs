/// Returns the leading elements of an array while the predicate returns true.
///
/// # Arguments
///
/// * `array` - The array to scan
/// * `predicate` - Function evaluated for each element until it first returns false
///
/// # Returns
///
/// A new vector of the leading elements that satisfy the predicate
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_take_while;
///
/// assert_eq!(umt_take_while(&[1, 2, 3, 4, 1], |n| *n < 3), vec![1, 2]);
/// ```
pub fn umt_take_while<T, F>(array: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    umt_take_while_indexed(array, |item, _| predicate(item))
}

/// Returns the leading elements of an array while the predicate returns true,
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
/// A new vector of the leading elements that satisfy the predicate
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_take_while_indexed;
///
/// assert_eq!(umt_take_while_indexed(&[5, 6, 7], |_, i| i < 2), vec![5, 6]);
/// ```
pub fn umt_take_while_indexed<T, F>(array: &[T], predicate: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T, usize) -> bool,
{
    let end = array
        .iter()
        .enumerate()
        .position(|(index, item)| !predicate(item, index))
        .unwrap_or(array.len());
    array[..end].to_vec()
}
