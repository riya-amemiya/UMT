//! Create a date from a Unix timestamp.

use chrono::{DateTime, Utc};

use super::unix_time_unit::UnixTimeUnit;

/// Creates a `DateTime<Utc>` from a Unix timestamp.
///
/// Unlike other Date helpers, this uses a real epoch timestamp rather than
/// wall-clock fields.
///
/// # Arguments
///
/// * `value` - Timestamp value
/// * `unit` - Seconds or milliseconds
///
/// # Examples
///
/// ```
/// use umt_rust::date::{umt_from_unix, UnixTimeUnit};
///
/// let date = umt_from_unix(0.0, UnixTimeUnit::Second);
/// assert_eq!(date.timestamp(), 0);
/// ```
pub fn umt_from_unix(value: f64, unit: UnixTimeUnit) -> DateTime<Utc> {
    match unit {
        UnixTimeUnit::Second => {
            DateTime::from_timestamp(value as i64, 0).expect("valid unix timestamp")
        }
        UnixTimeUnit::Millisecond => {
            DateTime::from_timestamp_millis(value as i64).expect("valid unix timestamp")
        }
    }
}
