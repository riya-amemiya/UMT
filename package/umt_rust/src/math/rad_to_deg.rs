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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rad_to_deg_pi() {
        let degrees = umt_rad_to_deg(PI);
        assert!((degrees - 180.0).abs() < 1e-10);
    }

    #[test]
    fn test_rad_to_deg_half_pi() {
        let degrees = umt_rad_to_deg(PI / 2.0);
        assert!((degrees - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_rad_to_deg_quarter_pi() {
        let degrees = umt_rad_to_deg(PI / 4.0);
        assert!((degrees - 45.0).abs() < 1e-10);
    }

    #[test]
    fn test_rad_to_deg_zero() {
        assert_eq!(umt_rad_to_deg(0.0), 0.0);
    }

    #[test]
    fn test_rad_to_deg_two_pi() {
        let degrees = umt_rad_to_deg(2.0 * PI);
        assert!((degrees - 360.0).abs() < 1e-10);
    }

    #[test]
    fn test_rad_to_deg_negative() {
        let degrees = umt_rad_to_deg(-PI);
        assert!((degrees - (-180.0)).abs() < 1e-10);
    }
}
