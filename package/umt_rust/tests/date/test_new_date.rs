//! Tests for the new_date module.

use umt_rust::date::{umt_new_date_int, umt_new_date_string};

#[test]
fn test_new_date_int_basic() {
    let date = umt_new_date_int(2025, 1, 1, None, None, None, None);
    assert!(date.is_some());
    let dt = date.unwrap();
    assert_eq!(dt.format("%Y").to_string(), "2025");
    assert_eq!(dt.format("%m").to_string(), "01");
    assert_eq!(dt.format("%d").to_string(), "01");
}

#[test]
fn test_new_date_int_with_time_components() {
    let date = umt_new_date_int(2025, 1, 1, Some(10), Some(30), Some(45), Some(500));
    assert!(date.is_some());
    let dt = date.unwrap();
    assert_eq!(dt.format("%Y").to_string(), "2025");
    assert_eq!(dt.format("%m").to_string(), "01");
    assert_eq!(dt.format("%d").to_string(), "01");
    assert_eq!(dt.format("%H").to_string(), "10");
    assert_eq!(dt.format("%M").to_string(), "30");
    assert_eq!(dt.format("%S").to_string(), "45");
}

#[test]
fn test_new_date_string_default_time() {
    let date = umt_new_date_string("2025-01-01", None, None, None, None, None);
    assert!(date.is_some());
    let dt = date.unwrap();
    // Without timezone offset, this should be midnight UTC
    assert_eq!(dt.format("%Y-%m-%d").to_string(), "2025-01-01");
    assert_eq!(dt.format("%H:%M:%S").to_string(), "00:00:00");
}

#[test]
fn test_new_date_string_with_time_components() {
    let date = umt_new_date_string("2025-01-01", Some("10"), Some("30"), Some("45"), Some("500"), Some(9));
    assert!(date.is_some());
    let dt = date.unwrap();
    // With JST (UTC+9) offset of 9 hours, 10:30 JST becomes 01:30 UTC
    assert_eq!(dt.format("%H:%M:%S").to_string(), "01:30:45");
}

#[test]
fn test_new_date_string_different_timezone_offsets() {
    let date_utc = umt_new_date_string("2025-01-01", Some("12"), Some("00"), Some("00"), Some("000"), Some(0));
    let date_jst = umt_new_date_string("2025-01-01", Some("12"), Some("00"), Some("00"), Some("000"), Some(9));

    assert!(date_utc.is_some());
    assert!(date_jst.is_some());

    // JST (UTC+9) date should be 9 hours behind UTC date when stored as UTC
    let diff = date_utc.unwrap().signed_duration_since(date_jst.unwrap());
    assert_eq!(diff.num_hours(), 9);
}

#[test]
fn test_new_date_int_invalid_month() {
    let date = umt_new_date_int(2021, 13, 1, None, None, None, None);
    assert!(date.is_none());
}

#[test]
fn test_new_date_int_invalid_day() {
    let date = umt_new_date_int(2021, 2, 30, None, None, None, None);
    assert!(date.is_none());
}

#[test]
fn test_new_date_string_invalid_format() {
    let date = umt_new_date_string("01-01-2021", None, None, None, None, None);
    assert!(date.is_none());
}
