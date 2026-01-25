//! Tests for the get_timezone_offset_string module.

use umt_rust::date::{
    umt_get_local_timezone_offset_string, umt_get_timezone_offset_string,
    umt_get_timezone_offset_string_compact,
};

#[test]
fn test_positive_timezone_offset() {
    // UTC+8 (480 minutes)
    assert_eq!(umt_get_timezone_offset_string(480), "+08:00");
}

#[test]
fn test_negative_timezone_offset() {
    // UTC-5 (-300 minutes)
    assert_eq!(umt_get_timezone_offset_string(-300), "-05:00");
}

#[test]
fn test_zero_timezone_offset() {
    // UTC (0 minutes)
    assert_eq!(umt_get_timezone_offset_string(0), "+00:00");
}

#[test]
fn test_timezone_offset_jst() {
    // JST is UTC+9 (540 minutes)
    assert_eq!(umt_get_timezone_offset_string(540), "+09:00");
}

#[test]
fn test_timezone_offset_compact() {
    assert_eq!(umt_get_timezone_offset_string_compact(540), "+0900");
    assert_eq!(umt_get_timezone_offset_string_compact(0), "+0000");
    assert_eq!(umt_get_timezone_offset_string_compact(-300), "-0500");
}

#[test]
fn test_local_timezone_offset_format() {
    let offset = umt_get_local_timezone_offset_string();
    // Verify it returns a valid format
    assert!(offset.starts_with('+') || offset.starts_with('-'));
    assert_eq!(offset.len(), 6);
    assert_eq!(&offset[3..4], ":");
}
