/// Converts temperature from Celsius to Kelvin.
///
/// # Arguments
///
/// * `celsius` - Temperature in Celsius.
///
/// # Returns
///
/// Temperature in Kelvin.
///
/// # Examples
///
/// ```
/// use umt_rust::math::umt_to_kelvin;
///
/// assert!((umt_to_kelvin(26.85) - 300.0).abs() < 1e-10);
/// assert!((umt_to_kelvin(0.0) - 273.15).abs() < 1e-10);
/// assert!((umt_to_kelvin(-273.15) - 0.0).abs() < 1e-10);
/// ```
#[inline]
pub fn umt_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_kelvin_room_temp() {
        assert!((umt_to_kelvin(26.85) - 300.0).abs() < 1e-10);
    }

    #[test]
    fn test_to_kelvin_freezing() {
        assert!((umt_to_kelvin(0.0) - 273.15).abs() < 1e-10);
    }

    #[test]
    fn test_to_kelvin_absolute_zero() {
        assert!((umt_to_kelvin(-273.15) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_to_kelvin_boiling() {
        assert!((umt_to_kelvin(100.0) - 373.15).abs() < 1e-10);
    }
}
