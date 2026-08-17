//! Convert a date to a Unix timestamp.

use chrono::{DateTime, Utc};

use super::unix_time_unit::UnixTimeUnit;

/// Converts a `DateTime<Utc>` to a Unix timestamp.
///
/// Seconds are floored. Unlike other Date helpers, this uses a real epoch
/// timestamp rather than wall-clock fields.
///
/// # Arguments
///
/// * `date` - Date to convert
/// * `unit` - Seconds or milliseconds
///
/// # Examples
///
/// ```
/// use umt_rust::date::{umt_from_unix, umt_to_unix, UnixTimeUnit};
///
/// let date = umt_from_unix(1_700_000_000.0, UnixTimeUnit::Second);
/// assert_eq!(umt_to_unix(&date, UnixTimeUnit::Second), 1_700_000_000.0);
/// ```
pub fn umt_to_unix(date: &DateTime<Utc>, unit: UnixTimeUnit) -> f64 {
    match unit {
        UnixTimeUnit::Second => (date.timestamp_millis() as f64 / 1000.0).floor(),
        UnixTimeUnit::Millisecond => date.timestamp_millis() as f64,
    }
}
