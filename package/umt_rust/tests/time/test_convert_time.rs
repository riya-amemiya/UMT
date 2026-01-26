use umt_rust::time::{umt_convert_time, umt_convert_time_from_str};

// Long format to long format tests
#[test]
fn test_converts_1_hour_to_seconds() {
    assert_eq!(
        umt_convert_time_from_str("1", "hours", "seconds"),
        Some(3600.0)
    );
    assert_eq!(umt_convert_time(1.0, "hours", "seconds"), Some(3600.0));
}

#[test]
fn test_converts_3600_seconds_to_hours() {
    assert_eq!(
        umt_convert_time_from_str("3600", "seconds", "hours"),
        Some(1.0)
    );
}

#[test]
fn test_converts_90_minutes_to_hours() {
    assert_eq!(
        umt_convert_time_from_str("90", "minutes", "hours"),
        Some(1.5)
    );
}

#[test]
fn test_converts_1_hour_to_milliseconds() {
    assert_eq!(
        umt_convert_time_from_str("1", "hours", "milliseconds"),
        Some(3_600_000.0)
    );
}

#[test]
fn test_converts_half_second_to_milliseconds() {
    assert_eq!(
        umt_convert_time_from_str("0.5", "seconds", "milliseconds"),
        Some(500.0)
    );
}

#[test]
fn test_converts_1000_milliseconds_to_seconds() {
    assert_eq!(
        umt_convert_time_from_str("1000", "milliseconds", "seconds"),
        Some(1.0)
    );
}

#[test]
fn test_converts_between_same_units() {
    assert_eq!(
        umt_convert_time_from_str("10", "seconds", "seconds"),
        Some(10.0)
    );
}

#[test]
fn test_converts_decimal_input() {
    assert_eq!(
        umt_convert_time_from_str("1.5", "hours", "minutes"),
        Some(90.0)
    );
}

#[test]
fn test_converts_zero() {
    assert_eq!(
        umt_convert_time_from_str("0", "hours", "seconds"),
        Some(0.0)
    );
}

#[test]
fn test_converts_very_large_numbers() {
    let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
    assert_eq!(
        umt_convert_time_from_str("1e9", "milliseconds", "hours"),
        Some(expected)
    );
}

// Short format to short format tests
#[test]
fn test_converts_1_hour_to_seconds_short() {
    assert_eq!(umt_convert_time_from_str("1", "h", "s"), Some(3600.0));
}

#[test]
fn test_converts_3600_seconds_to_hours_short() {
    assert_eq!(umt_convert_time_from_str("3600", "s", "h"), Some(1.0));
}

#[test]
fn test_converts_90_minutes_to_hours_short() {
    assert_eq!(umt_convert_time_from_str("90", "m", "h"), Some(1.5));
}

#[test]
fn test_converts_1_hour_to_milliseconds_short() {
    assert_eq!(umt_convert_time_from_str("1", "h", "ms"), Some(3_600_000.0));
}

#[test]
fn test_converts_half_second_to_milliseconds_short() {
    assert_eq!(umt_convert_time_from_str("0.5", "s", "ms"), Some(500.0));
}

#[test]
fn test_converts_1000_milliseconds_to_seconds_short() {
    assert_eq!(umt_convert_time_from_str("1000", "ms", "s"), Some(1.0));
}

#[test]
fn test_converts_between_same_units_short() {
    assert_eq!(umt_convert_time_from_str("10", "s", "s"), Some(10.0));
}

#[test]
fn test_converts_decimal_input_short() {
    assert_eq!(umt_convert_time_from_str("1.5", "h", "m"), Some(90.0));
}

#[test]
fn test_converts_zero_short() {
    assert_eq!(umt_convert_time_from_str("0", "h", "s"), Some(0.0));
}

#[test]
fn test_converts_very_large_numbers_short() {
    let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
    assert_eq!(umt_convert_time_from_str("1e9", "ms", "h"), Some(expected));
}

// Long format to short format tests
#[test]
fn test_converts_1_hour_to_seconds_long_to_short() {
    assert_eq!(umt_convert_time_from_str("1", "hours", "s"), Some(3600.0));
}

#[test]
fn test_converts_3600_seconds_to_hours_long_to_short() {
    assert_eq!(umt_convert_time_from_str("3600", "seconds", "h"), Some(1.0));
}

#[test]
fn test_converts_90_minutes_to_hours_long_to_short() {
    assert_eq!(umt_convert_time_from_str("90", "minutes", "h"), Some(1.5));
}

#[test]
fn test_converts_1_hour_to_milliseconds_long_to_short() {
    assert_eq!(
        umt_convert_time_from_str("1", "hours", "ms"),
        Some(3_600_000.0)
    );
}

#[test]
fn test_converts_half_second_to_milliseconds_long_to_short() {
    assert_eq!(
        umt_convert_time_from_str("0.5", "seconds", "ms"),
        Some(500.0)
    );
}

#[test]
fn test_converts_1000_milliseconds_to_seconds_long_to_short() {
    assert_eq!(
        umt_convert_time_from_str("1000", "milliseconds", "s"),
        Some(1.0)
    );
}

#[test]
fn test_converts_between_same_units_long_to_short() {
    assert_eq!(umt_convert_time_from_str("10", "seconds", "s"), Some(10.0));
}

#[test]
fn test_converts_decimal_input_long_to_short() {
    assert_eq!(umt_convert_time_from_str("1.5", "hours", "m"), Some(90.0));
}

#[test]
fn test_converts_zero_long_to_short() {
    assert_eq!(umt_convert_time_from_str("0", "hours", "s"), Some(0.0));
}

#[test]
fn test_converts_very_large_numbers_long_to_short() {
    let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
    assert_eq!(
        umt_convert_time_from_str("1e9", "milliseconds", "h"),
        Some(expected)
    );
}

// Short format to long format tests
#[test]
fn test_converts_1_hour_to_seconds_short_to_long() {
    assert_eq!(umt_convert_time_from_str("1", "h", "seconds"), Some(3600.0));
}

#[test]
fn test_converts_3600_seconds_to_hours_short_to_long() {
    assert_eq!(umt_convert_time_from_str("3600", "s", "hours"), Some(1.0));
}

#[test]
fn test_converts_90_minutes_to_hours_short_to_long() {
    assert_eq!(umt_convert_time_from_str("90", "m", "hours"), Some(1.5));
}

#[test]
fn test_converts_1_hour_to_milliseconds_short_to_long() {
    assert_eq!(
        umt_convert_time_from_str("1", "h", "milliseconds"),
        Some(3_600_000.0)
    );
}

#[test]
fn test_converts_half_second_to_milliseconds_short_to_long() {
    assert_eq!(
        umt_convert_time_from_str("0.5", "s", "milliseconds"),
        Some(500.0)
    );
}

#[test]
fn test_converts_1000_milliseconds_to_seconds_short_to_long() {
    assert_eq!(
        umt_convert_time_from_str("1000", "ms", "seconds"),
        Some(1.0)
    );
}

#[test]
fn test_converts_between_same_units_short_to_long() {
    assert_eq!(umt_convert_time_from_str("10", "s", "seconds"), Some(10.0));
}

#[test]
fn test_converts_decimal_input_short_to_long() {
    assert_eq!(umt_convert_time_from_str("1.5", "h", "minutes"), Some(90.0));
}

#[test]
fn test_converts_zero_short_to_long() {
    assert_eq!(umt_convert_time_from_str("0", "h", "seconds"), Some(0.0));
}

#[test]
fn test_converts_very_large_numbers_short_to_long() {
    let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
    assert_eq!(
        umt_convert_time_from_str("1e9", "ms", "hours"),
        Some(expected)
    );
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
    assert_eq!(
        umt_convert_time_from_str("invalid", "hours", "seconds"),
        None
    );
    assert_eq!(umt_convert_time_from_str("", "hours", "seconds"), None);
}

#[test]
fn test_numeric_value_directly() {
    assert_eq!(umt_convert_time(2.5, "hours", "minutes"), Some(150.0));
    assert_eq!(umt_convert_time(500.0, "ms", "s"), Some(0.5));
}

use umt_rust::time::*;

#[test]
fn test_convert_1000_milliseconds_to_seconds() {
    assert_eq!(
        umt_convert_time_from_str("1000", "milliseconds", "seconds"),
        Some(1.0)
    );
}

#[test]
fn test_convert_1000_milliseconds_to_seconds_short() {
    assert_eq!(umt_convert_time_from_str("1000", "ms", "s"), Some(1.0));
}

#[test]
fn test_convert_1_hour_to_milliseconds() {
    assert_eq!(
        umt_convert_time_from_str("1", "hours", "milliseconds"),
        Some(3_600_000.0)
    );
}

#[test]
fn test_convert_1_hour_to_milliseconds_short() {
    assert_eq!(umt_convert_time_from_str("1", "h", "ms"), Some(3_600_000.0));
}

#[test]
fn test_convert_1_hour_to_seconds() {
    assert_eq!(umt_convert_time(1.0, "hours", "seconds"), Some(3600.0));
    assert_eq!(
        umt_convert_time_from_str("1", "hours", "seconds"),
        Some(3600.0)
    );
}

#[test]
fn test_convert_1_hour_to_seconds_short() {
    assert_eq!(umt_convert_time_from_str("1", "h", "s"), Some(3600.0));
}

#[test]
fn test_convert_3600_seconds_to_hours() {
    assert_eq!(
        umt_convert_time_from_str("3600", "seconds", "hours"),
        Some(1.0)
    );
}

#[test]
fn test_convert_3600_seconds_to_hours_short() {
    assert_eq!(umt_convert_time_from_str("3600", "s", "h"), Some(1.0));
}

#[test]
fn test_convert_90_minutes_to_hours() {
    assert_eq!(
        umt_convert_time_from_str("90", "minutes", "hours"),
        Some(1.5)
    );
}

#[test]
fn test_convert_90_minutes_to_hours_short() {
    assert_eq!(umt_convert_time_from_str("90", "m", "h"), Some(1.5));
}

#[test]
fn test_convert_decimal_input() {
    assert_eq!(
        umt_convert_time_from_str("1.5", "hours", "minutes"),
        Some(90.0)
    );
}

#[test]
fn test_convert_decimal_input_short() {
    assert_eq!(umt_convert_time_from_str("1.5", "h", "m"), Some(90.0));
}

#[test]
fn test_convert_half_second_to_milliseconds() {
    assert_eq!(
        umt_convert_time_from_str("0.5", "seconds", "milliseconds"),
        Some(500.0)
    );
}

#[test]
fn test_convert_half_second_to_milliseconds_short() {
    assert_eq!(umt_convert_time_from_str("0.5", "s", "ms"), Some(500.0));
}

#[test]
fn test_convert_large_numbers() {
    let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
    assert_eq!(
        umt_convert_time_from_str("1e9", "milliseconds", "hours"),
        Some(expected)
    );
}

#[test]
fn test_convert_large_numbers_short() {
    let expected = 1_000_000_000.0 / (60.0 * 60.0 * 1000.0);
    assert_eq!(umt_convert_time_from_str("1e9", "ms", "h"), Some(expected));
}

#[test]
fn test_convert_long_to_short() {
    assert_eq!(umt_convert_time_from_str("1", "hours", "s"), Some(3600.0));
    assert_eq!(umt_convert_time_from_str("3600", "seconds", "h"), Some(1.0));
    assert_eq!(umt_convert_time_from_str("90", "minutes", "h"), Some(1.5));
    assert_eq!(
        umt_convert_time_from_str("1", "hours", "ms"),
        Some(3_600_000.0)
    );
    assert_eq!(
        umt_convert_time_from_str("0.5", "seconds", "ms"),
        Some(500.0)
    );
    assert_eq!(
        umt_convert_time_from_str("1000", "milliseconds", "s"),
        Some(1.0)
    );
    assert_eq!(umt_convert_time_from_str("10", "seconds", "s"), Some(10.0));
    assert_eq!(umt_convert_time_from_str("1.5", "hours", "m"), Some(90.0));
    assert_eq!(umt_convert_time_from_str("0", "hours", "s"), Some(0.0));
}

#[test]
fn test_convert_same_units() {
    assert_eq!(
        umt_convert_time_from_str("10", "seconds", "seconds"),
        Some(10.0)
    );
}

#[test]
fn test_convert_same_units_short() {
    assert_eq!(umt_convert_time_from_str("10", "s", "s"), Some(10.0));
}

#[test]
fn test_convert_short_to_long() {
    assert_eq!(umt_convert_time_from_str("1", "h", "seconds"), Some(3600.0));
    assert_eq!(umt_convert_time_from_str("3600", "s", "hours"), Some(1.0));
    assert_eq!(umt_convert_time_from_str("90", "m", "hours"), Some(1.5));
    assert_eq!(
        umt_convert_time_from_str("1", "h", "milliseconds"),
        Some(3_600_000.0)
    );
    assert_eq!(
        umt_convert_time_from_str("0.5", "s", "milliseconds"),
        Some(500.0)
    );
    assert_eq!(
        umt_convert_time_from_str("1000", "ms", "seconds"),
        Some(1.0)
    );
    assert_eq!(umt_convert_time_from_str("10", "s", "seconds"), Some(10.0));
    assert_eq!(umt_convert_time_from_str("1.5", "h", "minutes"), Some(90.0));
    assert_eq!(umt_convert_time_from_str("0", "h", "seconds"), Some(0.0));
}

#[test]
fn test_convert_zero() {
    assert_eq!(
        umt_convert_time_from_str("0", "hours", "seconds"),
        Some(0.0)
    );
}

#[test]
fn test_convert_zero_short() {
    assert_eq!(umt_convert_time_from_str("0", "h", "s"), Some(0.0));
}
