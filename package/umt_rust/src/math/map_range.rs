use super::{umt_inverse_lerp, umt_lerp};

/// Maps a value from one range to another without clamping.
///
/// # Arguments
///
/// * `value` - Value in the input range.
/// * `in_min` - Start of the input range.
/// * `in_max` - End of the input range.
/// * `out_min` - Start of the output range.
/// * `out_max` - End of the output range.
///
/// # Returns
///
/// The mapped value, or `out_min` when `in_min` equals `in_max`.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_map_range;
///
/// assert_eq!(umt_map_range(5.0, 0.0, 10.0, 0.0, 100.0), 50.0);
/// assert_eq!(umt_map_range(0.5, 0.0, 1.0, 10.0, 20.0), 15.0);
/// assert_eq!(umt_map_range(15.0, 0.0, 10.0, 0.0, 100.0), 150.0);
/// ```
#[inline]
pub fn umt_map_range(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    umt_lerp(out_min, out_max, umt_inverse_lerp(in_min, in_max, value))
}
