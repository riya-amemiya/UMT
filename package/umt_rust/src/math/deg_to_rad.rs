use std::f64::consts::PI;

/// Converts degrees to radians.
///
/// This function takes an angle in degrees as input and returns the equivalent angle in radians.
///
/// # Arguments
///
/// * `degrees` - The angle in degrees.
///
/// # Returns
///
/// The angle in radians.
pub fn umt_deg_to_rad(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
