/// Clamps a number between a minimum and maximum value.
///
/// This function takes a number and limits it to the given range [min, max].
/// If min > max, the result is max.
/// If any input is NaN, returns NaN.
///
/// # Arguments
///
/// * `value` - The number to clamp.
/// * `min` - The minimum bound.
/// * `max` - The maximum bound.
///
/// # Returns
///
/// The clamped number.
#[inline]
pub fn umt_clamp(value: f64, min: f64, max: f64) -> f64 {
    if value.is_nan() || min.is_nan() || max.is_nan() {
        return f64::NAN;
    }
    value.max(min).min(max)
}
