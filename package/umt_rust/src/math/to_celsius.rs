/// Converts temperature from Kelvin to Celsius.
///
/// # Arguments
///
/// * `kelvin` - Temperature in Kelvin.
///
/// # Returns
///
/// Temperature in Celsius.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_to_celsius;
///
/// assert!((umt_to_celsius(300.0) - 26.85).abs() < 1e-10);
/// assert!((umt_to_celsius(273.15) - 0.0).abs() < 1e-10);
/// assert!((umt_to_celsius(0.0) - (-273.15)).abs() < 1e-10);
/// ```
#[inline]
pub fn umt_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_celsius_300k() {
        assert!((umt_to_celsius(300.0) - 26.85).abs() < 1e-10);
    }

    #[test]
    fn test_to_celsius_freezing() {
        assert!((umt_to_celsius(273.15) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_to_celsius_absolute_zero() {
        assert!((umt_to_celsius(0.0) - (-273.15)).abs() < 1e-10);
    }

    #[test]
    fn test_to_celsius_boiling() {
        assert!((umt_to_celsius(373.15) - 100.0).abs() < 1e-10);
    }
}
