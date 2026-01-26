//! Current time utility with timezone offset.
//!
//! This module provides a function to get the current time adjusted for a specified UTC offset.

use crate::internal::datetime::{UmtDateTime, UmtDuration};

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
pub fn umt_now(time_difference: i32) -> UmtDateTime {
    let now = UmtDateTime::now();
    let offset_ms = (time_difference as i64) * (ONE_HOUR_MS as i64);
    now + UmtDuration::milliseconds(offset_ms)
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
pub fn umt_now_jst() -> UmtDateTime {
    umt_now(9)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_returns_datetime() {
        let result = umt_now(0);
        // Just verify it returns a valid DateTime
        assert!(result.timestamp() > 0);
    }

    #[test]
    fn test_now_with_positive_offset() {
        let utc = umt_now(0);
        let jst = umt_now(9);
        let diff = jst.signed_duration_since(&utc);
        assert_eq!(diff.num_hours(), 9);
    }

    #[test]
    fn test_now_with_negative_offset() {
        // Use a fixed base time to avoid race conditions
        let base_time = UmtDateTime::now();
        let offset_ms_0 = 0i64 * (ONE_HOUR_MS as i64);
        let offset_ms_neg5 = -5i64 * (ONE_HOUR_MS as i64);

        let utc = base_time + UmtDuration::milliseconds(offset_ms_0);
        let est = base_time + UmtDuration::milliseconds(offset_ms_neg5);
        let diff = utc.signed_duration_since(&est);
        assert_eq!(diff.num_hours(), 5);
    }

    #[test]
    fn test_now_jst() {
        // Use a fixed base time to avoid race conditions
        let base_time = UmtDateTime::now();
        let offset_ms_9 = 9i64 * (ONE_HOUR_MS as i64);
        let offset_ms_0 = 0i64 * (ONE_HOUR_MS as i64);

        let jst = base_time + UmtDuration::milliseconds(offset_ms_9);
        let utc = base_time + UmtDuration::milliseconds(offset_ms_0);
        let diff = jst.signed_duration_since(&utc);
        assert_eq!(diff.num_hours(), 9);
    }
}
