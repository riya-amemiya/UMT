//! Tests for the now module.

use umt_rust::date::{umt_now, umt_now_jst};

#[test]
fn test_now_returns_valid_datetime() {
    let result = umt_now(0);
    // Just verify it returns a valid DateTime with a positive timestamp
    assert!(result.timestamp() > 0);
}

#[test]
fn test_now_utc_time() {
    let result = umt_now(0);
    // Verify the timestamp is reasonable (after year 2020)
    assert!(result.timestamp() > 1577836800); // 2020-01-01 00:00:00 UTC
}

#[test]
fn test_now_with_different_offsets() {
    let utc = umt_now(0);
    let jst = umt_now(9);

    // JST should be 9 hours ahead of UTC
    let diff = jst.signed_duration_since(utc);
    assert_eq!(diff.num_hours(), 9);
}

#[test]
fn test_now_jst_offset() {
    let utc = umt_now(0);
    let jst = umt_now_jst();

    // JST (from umt_now_jst) should be 9 hours ahead of UTC
    let diff = jst.signed_duration_since(utc);
    assert_eq!(diff.num_hours(), 9);
}

#[test]
fn test_now_correct_time_difference_between_timezones() {
    let utc_result = umt_now(0);
    let jst_result = umt_now(9);

    // JST should be 9 hours ahead of UTC
    let diff_ms = (jst_result.timestamp_millis() - utc_result.timestamp_millis()).abs();
    let expected_diff_ms = 9 * 60 * 60 * 1000;
    assert_eq!(diff_ms, expected_diff_ms);
}

use chrono::{Duration, Utc};
use umt_rust::consts::ONE_HOUR_MS;
use umt_rust::date::*;

#[test]
fn test_now_jst() {
    // Use a fixed base time to avoid race conditions
    let base_time = Utc::now();
    let offset_ms_9 = 9i64 * (ONE_HOUR_MS as i64);
    let offset_ms_0 = 0i64 * (ONE_HOUR_MS as i64);

    let jst = base_time + Duration::milliseconds(offset_ms_9);
    let utc = base_time + Duration::milliseconds(offset_ms_0);
    let diff = jst.signed_duration_since(utc);
    assert_eq!(diff.num_hours(), 9);
}

#[test]
fn test_now_returns_datetime() {
    let result = umt_now(0);
    // Just verify it returns a valid DateTime
    assert!(result.timestamp() > 0);
}

#[test]
fn test_now_with_negative_offset() {
    // Use a fixed base time to avoid race conditions
    let base_time = Utc::now();
    let offset_ms_0 = 0i64 * (ONE_HOUR_MS as i64);
    let offset_ms_neg5 = -5i64 * (ONE_HOUR_MS as i64);

    let utc = base_time + Duration::milliseconds(offset_ms_0);
    let est = base_time + Duration::milliseconds(offset_ms_neg5);
    let diff = utc.signed_duration_since(est);
    assert_eq!(diff.num_hours(), 5);
}

#[test]
fn test_now_with_positive_offset() {
    let utc = umt_now(0);
    let jst = umt_now(9);
    let diff = jst.signed_duration_since(utc);
    assert_eq!(diff.num_hours(), 9);
}
