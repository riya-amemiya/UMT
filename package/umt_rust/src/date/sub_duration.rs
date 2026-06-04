//! Subtract a duration from a date.

use chrono::{DateTime, Utc};

use super::add_duration::umt_add_duration;
use super::ms_by_unit::DurationUnit;

/// Subtracts a duration from a date and returns a new `DateTime<Utc>`.
///
/// Equivalent to `umt_add_duration(date, -amount, unit)`.
///
/// # Arguments
///
/// * `date` - Base date
/// * `amount` - Amount to subtract
/// * `unit` - Unit of the amount
///
/// # Returns
///
/// A new `DateTime<Utc>` with the duration subtracted.
///
/// # Examples
///
/// ```
/// use chrono::{Datelike, TimeZone, Utc};
/// use umt_rust::date::{umt_sub_duration, DurationUnit};
///
/// let base = Utc.with_ymd_and_hms(2025, 3, 31, 0, 0, 0).unwrap();
/// let result = umt_sub_duration(&base, 1, DurationUnit::Month);
/// assert_eq!(result.month(), 2);
/// assert_eq!(result.day(), 28);
/// ```
#[inline]
pub fn umt_sub_duration(date: &DateTime<Utc>, amount: i64, unit: DurationUnit) -> DateTime<Utc> {
    umt_add_duration(date, -amount, unit)
}
