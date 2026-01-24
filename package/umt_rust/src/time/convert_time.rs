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
    let milliseconds = value * from.to_milliseconds_rate();
    Some(milliseconds / to.to_milliseconds_rate())
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

#[cfg(test)]
mod tests {
    use super::*;

    // Long format to long format tests
    #[test]
    fn test_convert_1_hour_to_seconds() {
        assert_eq!(umt_convert_time(1.0, "hours", "seconds"), Some(3600.0));
        assert_eq!(umt_convert_time_from_str("1", "hours", "seconds"), Some(3600.0));
    }

    #[test]
    fn test_convert_3600_seconds_to_hours() {
        assert_eq!(umt_convert_time_from_str("3600", "seconds", "hours"), Some(1.0));
    }

    #[test]
    fn test_convert_90_minutes_to_hours() {
        assert_eq!(umt_convert_time_from_str("90", "minutes", "hours"), Some(1.5));
    }

    #[test]
    fn test_convert_1_hour_to_milliseconds() {
        assert_eq!(umt_convert_time_from_str("1", "hours", "milliseconds"), Some(3_600_000.0));
    }

    #[test]
    fn test_convert_half_second_to_milliseconds() {
        assert_eq!(umt_convert_time_from_str("0.5", "seconds", "milliseconds"), Some(500.0));
    }

    #[test]
    fn test_convert_1000_milliseconds_to_seconds() {
        assert_eq!(umt_convert_time_from_str("1000", "milliseconds", "seconds"), Some(1.0));
    }

    #[test]
    fn test_convert_same_units() {
        assert_eq!(umt_convert_time_from_str("10", "seconds", "seconds"), Some(10.0));
    }

    #[test]
    fn test_convert_decimal_input() {
        assert_eq!(umt_convert_time_from_str("1.5", "hours", "minutes"), Some(90.0));
    }

    #[test]
    fn test_convert_zero() {
        assert_eq!(umt_convert_time_from_str("0", "hours", "seconds"), Some(0.0));
    }

    #[test]
    fn test_convert_large_numbers() {
        let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
        assert_eq!(umt_convert_time_from_str("1e9", "milliseconds", "hours"), Some(expected));
    }

    // Short format to short format tests
    #[test]
    fn test_convert_1_hour_to_seconds_short() {
        assert_eq!(umt_convert_time_from_str("1", "h", "s"), Some(3600.0));
    }

    #[test]
    fn test_convert_3600_seconds_to_hours_short() {
        assert_eq!(umt_convert_time_from_str("3600", "s", "h"), Some(1.0));
    }

    #[test]
    fn test_convert_90_minutes_to_hours_short() {
        assert_eq!(umt_convert_time_from_str("90", "m", "h"), Some(1.5));
    }

    #[test]
    fn test_convert_1_hour_to_milliseconds_short() {
        assert_eq!(umt_convert_time_from_str("1", "h", "ms"), Some(3_600_000.0));
    }

    #[test]
    fn test_convert_half_second_to_milliseconds_short() {
        assert_eq!(umt_convert_time_from_str("0.5", "s", "ms"), Some(500.0));
    }

    #[test]
    fn test_convert_1000_milliseconds_to_seconds_short() {
        assert_eq!(umt_convert_time_from_str("1000", "ms", "s"), Some(1.0));
    }

    #[test]
    fn test_convert_same_units_short() {
        assert_eq!(umt_convert_time_from_str("10", "s", "s"), Some(10.0));
    }

    #[test]
    fn test_convert_decimal_input_short() {
        assert_eq!(umt_convert_time_from_str("1.5", "h", "m"), Some(90.0));
    }

    #[test]
    fn test_convert_zero_short() {
        assert_eq!(umt_convert_time_from_str("0", "h", "s"), Some(0.0));
    }

    #[test]
    fn test_convert_large_numbers_short() {
        let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
        assert_eq!(umt_convert_time_from_str("1e9", "ms", "h"), Some(expected));
    }

    // Long format to short format tests
    #[test]
    fn test_convert_long_to_short() {
        assert_eq!(umt_convert_time_from_str("1", "hours", "s"), Some(3600.0));
        assert_eq!(umt_convert_time_from_str("3600", "seconds", "h"), Some(1.0));
        assert_eq!(umt_convert_time_from_str("90", "minutes", "h"), Some(1.5));
        assert_eq!(umt_convert_time_from_str("1", "hours", "ms"), Some(3_600_000.0));
        assert_eq!(umt_convert_time_from_str("0.5", "seconds", "ms"), Some(500.0));
        assert_eq!(umt_convert_time_from_str("1000", "milliseconds", "s"), Some(1.0));
        assert_eq!(umt_convert_time_from_str("10", "seconds", "s"), Some(10.0));
        assert_eq!(umt_convert_time_from_str("1.5", "hours", "m"), Some(90.0));
        assert_eq!(umt_convert_time_from_str("0", "hours", "s"), Some(0.0));
    }

    // Short format to long format tests
    #[test]
    fn test_convert_short_to_long() {
        assert_eq!(umt_convert_time_from_str("1", "h", "seconds"), Some(3600.0));
        assert_eq!(umt_convert_time_from_str("3600", "s", "hours"), Some(1.0));
        assert_eq!(umt_convert_time_from_str("90", "m", "hours"), Some(1.5));
        assert_eq!(umt_convert_time_from_str("1", "h", "milliseconds"), Some(3_600_000.0));
        assert_eq!(umt_convert_time_from_str("0.5", "s", "milliseconds"), Some(500.0));
        assert_eq!(umt_convert_time_from_str("1000", "ms", "seconds"), Some(1.0));
        assert_eq!(umt_convert_time_from_str("10", "s", "seconds"), Some(10.0));
        assert_eq!(umt_convert_time_from_str("1.5", "h", "minutes"), Some(90.0));
        assert_eq!(umt_convert_time_from_str("0", "h", "seconds"), Some(0.0));
    }

    // Error handling tests
    #[test]
    fn test_invalid_from_unit() {
        assert_eq!(umt_convert_time(1.0, "invalid", "seconds"), None);
    }

    #[test]
    fn test_invalid_to_unit() {
        assert_eq!(umt_convert_time(1.0, "hours", "invalid"), None);
    }

    #[test]
    fn test_invalid_value_string() {
        assert_eq!(umt_convert_time_from_str("invalid", "hours", "seconds"), None);
    }
}
