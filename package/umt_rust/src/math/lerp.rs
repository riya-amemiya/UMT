/// Linearly interpolates between two values.
///
/// # Arguments
///
/// * `start` - Value returned when `t` is 0.
/// * `end` - Value returned when `t` is 1.
/// * `t` - Interpolation factor; values outside 0..1 extrapolate.
///
/// # Returns
///
/// `start + (end - start) * t`.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_lerp;
///
/// assert_eq!(umt_lerp(0.0, 10.0, 0.5), 5.0);
/// assert_eq!(umt_lerp(10.0, 20.0, 0.25), 12.5);
/// assert_eq!(umt_lerp(0.0, 10.0, 2.0), 20.0);
/// ```
#[inline]
pub fn umt_lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}
