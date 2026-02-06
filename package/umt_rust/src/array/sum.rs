/// Returns the sum of an array of numbers.
///
/// Uses precision-corrected addition to avoid floating point errors.
///
/// # Arguments
///
/// * `array` - Array of numbers
///
/// # Returns
///
/// Sum of the array elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_sum;
///
/// assert_eq!(umt_sum(&[1.0, 2.0, 3.0]), 6.0);
/// assert_eq!(umt_sum(&[]), 0.0);
/// ```
#[inline]
pub fn umt_sum(array: &[f64]) -> f64 {
    crate::math::umt_addition(array)
}

/// Returns the sum of an array of integers.
///
/// # Arguments
///
/// * `array` - Array of integers
///
/// # Returns
///
/// Sum of the array elements
///
/// # Examples
///
/// ```
/// use umt_rust::array::umt_sum_i64;
///
/// assert_eq!(umt_sum_i64(&[1, 2, 3]), 6);
/// assert_eq!(umt_sum_i64(&[]), 0);
/// ```
#[inline]
pub fn umt_sum_i64(array: &[i64]) -> i64 {
    array.iter().sum()
}
