//! Business-day check utility.

use chrono::{DateTime, Utc};

use super::is_same_day::umt_is_same_day;
use super::is_weekend::umt_is_weekend;

/// Returns `true` when the given date is a weekday and not in the holidays list.
///
/// The `DateTime<Utc>` fields are treated as the working wall-clock,
/// matching the local-time semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `date` - The date to check
/// * `holidays` - List of holiday dates (compared by calendar day)
///
/// # Returns
///
/// `true` when the date is a weekday and not a listed holiday.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_is_business_day;
///
/// // 2025-04-21 is a Monday.
/// let monday = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
/// assert!(umt_is_business_day(&monday, &[]));
/// assert!(!umt_is_business_day(&monday, &[monday]));
/// ```
pub fn umt_is_business_day(date: &DateTime<Utc>, holidays: &[DateTime<Utc>]) -> bool {
    if umt_is_weekend(date) {
        return false;
    }
    !holidays
        .iter()
        .any(|holiday| umt_is_same_day(holiday, date))
}
