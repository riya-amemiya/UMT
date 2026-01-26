/// Returns the first element of an array, or None if the array is empty.
///
/// # Arguments
///
/// * `array` - The input array
///
/// # Returns
///
/// The first element of the array, or None if the array is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_first;
///
/// assert_eq!(umt_first(&[1, 2, 3]), Some(&1));
/// assert_eq!(umt_first::<i32>(&[]), None);
/// ```
#[inline]
pub fn umt_first<T>(array: &[T]) -> Option<&T> {
    array.first()
}
