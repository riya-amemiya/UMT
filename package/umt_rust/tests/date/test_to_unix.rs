//! Tests for the to_unix module.

use umt_rust::date::{UnixTimeUnit, umt_from_unix, umt_to_unix};

#[test]
fn test_returns_floored_seconds() {
    let date = umt_from_unix(0.0, UnixTimeUnit::Second);
    assert_eq!(umt_to_unix(&date, UnixTimeUnit::Second), 0.0);
    let date = umt_from_unix(1_700_000_000_999.0, UnixTimeUnit::Millisecond);
    assert_eq!(umt_to_unix(&date, UnixTimeUnit::Second), 1_700_000_000.0);
}

#[test]
fn test_returns_milliseconds() {
    let date = umt_from_unix(1_700_000_000_123.0, UnixTimeUnit::Millisecond);
    assert_eq!(
        umt_to_unix(&date, UnixTimeUnit::Millisecond),
        1_700_000_000_123.0
    );
}

#[test]
fn test_floors_partial_seconds() {
    let date = umt_from_unix(1500.0, UnixTimeUnit::Millisecond);
    assert_eq!(umt_to_unix(&date, UnixTimeUnit::Second), 1.0);
}
