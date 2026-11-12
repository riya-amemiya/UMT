/// Calculates the interpolation factor of a value between two values.
///
/// # Arguments
///
/// * `start` - Value that maps to 0.
/// * `end` - Value that maps to 1.
/// * `value` - Value to locate; values outside the range give factors outside 0..1.
///
/// # Returns
///
/// `(value - start) / (end - start)`, or 0.0 when `start` equals `end`.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_inverse_lerp;
///
/// assert_eq!(umt_inverse_lerp(0.0, 10.0, 5.0), 0.5);
/// assert_eq!(umt_inverse_lerp(10.0, 20.0, 12.5), 0.25);
/// assert_eq!(umt_inverse_lerp(5.0, 5.0, 7.0), 0.0);
/// ```
#[inline]
pub fn umt_inverse_lerp(start: f64, end: f64, value: f64) -> f64 {
    if start == end {
        0.0
    } else {
        (value - start) / (end - start)
    }
}
