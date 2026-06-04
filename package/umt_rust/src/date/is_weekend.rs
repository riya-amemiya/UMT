//! Weekend check utility.

use chrono::{DateTime, Datelike, Utc};

/// Returns `true` when the given date is Saturday or Sunday.
///
/// The `DateTime<Utc>` fields are treated as the working wall-clock,
/// matching the local-time semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `date` - The date to check
///
/// # Returns
///
/// `true` if Saturday or Sunday, `false` otherwise.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_is_weekend;
///
/// // 2025-04-19 is a Saturday.
/// let saturday = Utc.with_ymd_and_hms(2025, 4, 19, 0, 0, 0).unwrap();
/// assert!(umt_is_weekend(&saturday));
///
/// // 2025-04-21 is a Monday.
/// let monday = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
/// assert!(!umt_is_weekend(&monday));
/// ```
#[inline]
pub fn umt_is_weekend(date: &DateTime<Utc>) -> bool {
    let day_of_week = date.weekday().num_days_from_sunday();
    day_of_week == 0 || day_of_week == 6
}
