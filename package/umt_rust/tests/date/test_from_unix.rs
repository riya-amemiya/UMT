//! Tests for the from_unix module.

use umt_rust::date::{UnixTimeUnit, umt_from_unix};

#[test]
fn test_creates_datetime_from_seconds() {
    let date = umt_from_unix(0.0, UnixTimeUnit::Second);
    assert_eq!(date.timestamp(), 0);
    let date = umt_from_unix(1_700_000_000.0, UnixTimeUnit::Second);
    assert_eq!(date.timestamp(), 1_700_000_000);
}

#[test]
fn test_creates_datetime_from_milliseconds() {
    let date = umt_from_unix(1_700_000_000_123.0, UnixTimeUnit::Millisecond);
    assert_eq!(date.timestamp_millis(), 1_700_000_000_123);
}

#[test]
fn test_keeps_fractional_seconds_as_milliseconds() {
    let date = umt_from_unix(1.5, UnixTimeUnit::Second);
    assert_eq!(date.timestamp_millis(), 1500);
    let date = umt_from_unix(-1.5, UnixTimeUnit::Second);
    assert_eq!(date.timestamp_millis(), -1500);
}

#[test]
fn test_truncates_fractional_milliseconds_toward_zero() {
    let date = umt_from_unix(1.9995, UnixTimeUnit::Second);
    assert_eq!(date.timestamp_millis(), 1999);
    let date = umt_from_unix(1_700_000_000_123.7, UnixTimeUnit::Millisecond);
    assert_eq!(date.timestamp_millis(), 1_700_000_000_123);
}
