use crate::time::normalize_time_unit::TimeUnit;

/// Converts time between different units.
///
/// This function converts a time value from one unit to another.
/// It accepts both long format (milliseconds, seconds, minutes, hours) and
/// short format (ms, s, m, h) unit specifiers.
///
/// # Arguments
///
/// * `value` - The numeric value to convert.
/// * `from_unit` - The source time unit (long or short format).
/// * `to_unit` - The target time unit (long or short format).
///
/// # Returns
///
/// `Some(f64)` with the converted value, or `None` if either unit is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::time::umt_convert_time;
///
/// // Convert 1 hour to seconds
/// assert_eq!(umt_convert_time(1.0, "hours", "seconds"), Some(3600.0));
///
/// // Convert using short format
/// assert_eq!(umt_convert_time(1.0, "h", "s"), Some(3600.0));
///
/// // Mixed formats work too
/// assert_eq!(umt_convert_time(1.0, "hours", "s"), Some(3600.0));
/// ```
pub fn umt_convert_time(value: f64, from_unit: &str, to_unit: &str) -> Option<f64> {
    let from = TimeUnit::from_str(from_unit)?;
    let to = TimeUnit::from_str(to_unit)?;

    // Convert to milliseconds first, then to target unit
    // Use precision-corrected math to match TypeScript version
    let milliseconds = crate::math::umt_multiplication(&[value, from.to_milliseconds_rate()]);
    Some(crate::math::umt_division(
        milliseconds,
        to.to_milliseconds_rate(),
    ))
}

/// Converts time between different units from a string value.
///
/// This is a convenience function that parses a string value before converting.
///
/// # Arguments
///
/// * `value` - The string value to convert (will be parsed as f64).
/// * `from_unit` - The source time unit (long or short format).
/// * `to_unit` - The target time unit (long or short format).
///
/// # Returns
///
/// `Some(f64)` with the converted value, or `None` if the value cannot be parsed
/// or either unit is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::time::umt_convert_time_from_str;
///
/// assert_eq!(umt_convert_time_from_str("1", "hours", "seconds"), Some(3600.0));
/// assert_eq!(umt_convert_time_from_str("1.5", "hours", "minutes"), Some(90.0));
/// assert_eq!(umt_convert_time_from_str("invalid", "hours", "seconds"), None);
/// ```
pub fn umt_convert_time_from_str(value: &str, from_unit: &str, to_unit: &str) -> Option<f64> {
    let numeric_value: f64 = value.parse().ok()?;
    umt_convert_time(numeric_value, from_unit, to_unit)
}
