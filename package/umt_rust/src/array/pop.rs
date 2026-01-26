/// Returns the last element of an array, or None if the array is empty.
/// This function does not modify the original array.
///
/// # Arguments
///
/// * `array` - The input array
///
/// # Returns
///
/// The last element of the array, or None if the array is empty.
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_pop;
///
/// assert_eq!(umt_pop(&[1, 2, 3]), Some(&3));
/// assert_eq!(umt_pop::<i32>(&[]), None);
/// ```
#[inline]
pub fn umt_pop<T>(array: &[T]) -> Option<&T> {
    array.last()
}
