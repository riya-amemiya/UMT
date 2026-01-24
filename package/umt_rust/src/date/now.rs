//! Current time utilities with timezone offset support.

use chrono::{DateTime, Duration, FixedOffset, Utc};

/// One hour in milliseconds.
pub const ONE_HOUR_MS: i64 = 3_600_000;

/// Gets the current time with a specified UTC offset.
///
/// This function returns the current time adjusted for the specified timezone offset,
/// regardless of the local system timezone.
///
/// # Arguments
///
/// * `time_difference` - Hours offset from UTC (default: 9 for Japan Standard Time).
///                       Can be negative for timezones west of UTC.
///
/// # Returns
///
/// A DateTime with FixedOffset representing the current time in the specified timezone.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_now;
///
/// // Get current time in JST (UTC+9)
/// let jst_time = umt_now(9);
///
/// // Get current time in UTC
/// let utc_time = umt_now(0);
///
/// // Get current time in EST (UTC-5)
/// let est_time = umt_now(-5);
/// ```
pub fn umt_now(time_difference: i32) -> DateTime<FixedOffset> {
    let utc_now = Utc::now();
    let offset_seconds = time_difference * 3600;
    let fixed_offset = FixedOffset::east_opt(offset_seconds).unwrap_or_else(|| {
        // Fallback for invalid offsets (shouldn't happen with valid hour values)
        FixedOffset::east_opt(0).unwrap()
    });

    // Add the time difference to get the "apparent" time in that timezone
    let adjusted_utc = utc_now + Duration::hours(time_difference as i64);
    DateTime::from_naive_utc_and_offset(adjusted_utc.naive_utc(), fixed_offset)
}

/// Gets the current time in JST (UTC+9) - the default timezone.
///
/// # Returns
///
/// A DateTime with FixedOffset representing the current time in Japan Standard Time.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_now_jst;
///
/// let jst_time = umt_now_jst();
/// ```
#[inline]
pub fn umt_now_jst() -> DateTime<FixedOffset> {
    umt_now(9)
}

/// Gets the current time in UTC.
///
/// # Returns
///
/// A DateTime with FixedOffset representing the current time in UTC.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_now_utc;
///
/// let utc_time = umt_now_utc();
/// ```
#[inline]
pub fn umt_now_utc() -> DateTime<FixedOffset> {
    umt_now(0)
}

/// Gets the current timestamp in milliseconds.
///
/// # Returns
///
/// The current Unix timestamp in milliseconds.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_now_timestamp_ms;
///
/// let timestamp = umt_now_timestamp_ms();
/// ```
#[inline]
pub fn umt_now_timestamp_ms() -> i64 {
    Utc::now().timestamp_millis()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_jst_offset() {
        let jst = umt_now(9);
        let utc = umt_now(0);

        // JST should be 9 hours ahead of UTC
        let diff_ms = jst.timestamp_millis() - utc.timestamp_millis();
        assert_eq!(diff_ms, 9 * ONE_HOUR_MS);
    }

    #[test]
    fn test_now_utc_offset() {
        let utc = umt_now(0);
        let actual_utc = Utc::now();

        // Should be very close to actual UTC (within 1 second)
        let diff = (utc.timestamp() - actual_utc.timestamp()).abs();
        assert!(diff < 2);
    }

    #[test]
    fn test_now_different_timezones() {
        let utc = umt_now(0);
        let cet = umt_now(1);
        let cst = umt_now(8);

        // CET should be 1 hour ahead of UTC
        assert_eq!(cet.timestamp_millis() - utc.timestamp_millis(), ONE_HOUR_MS);

        // CST should be 8 hours ahead of UTC
        assert_eq!(
            cst.timestamp_millis() - utc.timestamp_millis(),
            8 * ONE_HOUR_MS
        );
    }

    #[test]
    fn test_time_difference_between_timezones() {
        let utc = umt_now(0);
        let jst = umt_now(9);

        // JST should be 9 hours ahead of UTC
        assert_eq!(
            jst.timestamp_millis() - utc.timestamp_millis(),
            9 * ONE_HOUR_MS
        );
    }

    #[test]
    fn test_now_negative_offset() {
        let utc = umt_now(0);
        let est = umt_now(-5);

        // EST should be 5 hours behind UTC (negative difference)
        assert_eq!(
            est.timestamp_millis() - utc.timestamp_millis(),
            -5 * ONE_HOUR_MS
        );
    }

    #[test]
    fn test_now_jst_convenience() {
        let jst1 = umt_now_jst();
        let jst2 = umt_now(9);

        // Should be very close (within 1 second)
        let diff = (jst1.timestamp() - jst2.timestamp()).abs();
        assert!(diff < 2);
    }

    #[test]
    fn test_now_utc_convenience() {
        let utc1 = umt_now_utc();
        let utc2 = umt_now(0);

        // Should be very close (within 1 second)
        let diff = (utc1.timestamp() - utc2.timestamp()).abs();
        assert!(diff < 2);
    }

    #[test]
    fn test_timestamp_ms() {
        let ts = umt_now_timestamp_ms();
        let utc_ts = Utc::now().timestamp_millis();

        // Should be very close (within 1 second)
        let diff = (ts - utc_ts).abs();
        assert!(diff < 1000);
    }
}
