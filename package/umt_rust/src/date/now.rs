//! Current time utility with timezone offset.
//!
//! This module provides a function to get the current time adjusted for a specified UTC offset.

use chrono::{DateTime, Duration, Utc};

use crate::consts::ONE_HOUR_MS;

/// Get the current time with a specified UTC offset.
///
/// Returns the current date and time adjusted for the specified UTC offset,
/// regardless of the local timezone.
///
/// # Arguments
///
/// * `time_difference` - Hours offset from UTC (default: 9 for Japan Standard Time)
///
/// # Returns
///
/// Current date and time adjusted for the specified UTC offset
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_now;
///
/// let jst_time = umt_now(9);   // Current time in JST (UTC+9)
/// let utc_time = umt_now(0);   // Current time in UTC
/// let cet_time = umt_now(1);   // Current time in UTC+1
/// ```
#[inline]
pub fn umt_now(time_difference: i32) -> DateTime<Utc> {
    let now = Utc::now();
    let offset_ms = (time_difference as i64) * (ONE_HOUR_MS as i64);
    now + Duration::milliseconds(offset_ms)
}

/// Get the current time in Japan Standard Time (UTC+9).
///
/// Convenience function that calls `umt_now(9)`.
///
/// # Returns
///
/// Current date and time in JST
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_now_jst;
///
/// let jst_time = umt_now_jst();
/// ```
#[inline]
pub fn umt_now_jst() -> DateTime<Utc> {
    umt_now(9)
}
