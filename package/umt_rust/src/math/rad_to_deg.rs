use std::f64::consts::PI;

/// Converts radians to degrees.
///
/// Uses the formula: degrees = radians * (180/pi)
///
/// # Arguments
///
/// * `x` - Angle in radians.
///
/// # Returns
///
/// Angle in degrees.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_rad_to_deg;
/// use std::f64::consts::PI;
///
/// let degrees = umt_rad_to_deg(PI);
/// assert!((degrees - 180.0).abs() < 1e-10);
/// ```
#[inline]
pub fn umt_rad_to_deg(x: f64) -> f64 {
    x * (180.0 / PI)
}
