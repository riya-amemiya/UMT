//! Same-day comparison utility.

use chrono::{DateTime, Datelike, Utc};

/// Returns `true` when two dates represent the same calendar day.
///
/// The `DateTime<Utc>` fields are treated as the working wall-clock,
/// matching the local-time semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `left` - First date
/// * `right` - Second date
///
/// # Returns
///
/// `true` if same year/month/day, `false` otherwise.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_is_same_day;
///
/// let a = Utc.with_ymd_and_hms(2025, 4, 15, 1, 0, 0).unwrap();
/// let b = Utc.with_ymd_and_hms(2025, 4, 15, 23, 0, 0).unwrap();
/// assert!(umt_is_same_day(&a, &b));
///
/// let c = Utc.with_ymd_and_hms(2025, 4, 16, 0, 0, 0).unwrap();
/// assert!(!umt_is_same_day(&a, &c));
/// ```
#[inline]
pub fn umt_is_same_day(left: &DateTime<Utc>, right: &DateTime<Utc>) -> bool {
    left.year() == right.year() && left.month() == right.month() && left.day() == right.day()
}
