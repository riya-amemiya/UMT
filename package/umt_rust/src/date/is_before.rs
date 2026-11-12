//! Granular "before" comparison for dates.

use chrono::{DateTime, Utc};

use super::start_of::{DateBoundaryUnit, umt_start_of};

/// Returns `true` when `date` is strictly before `compare` at the given unit granularity.
///
/// Without a unit, compares exact timestamps (millisecond precision). With a
/// unit, both dates are truncated with [`umt_start_of`] before comparison.
/// Week boundaries follow the Sunday-start week, matching [`umt_start_of`].
///
/// # Arguments
///
/// * `date` - Date to test
/// * `compare` - Date to compare against
/// * `unit` - Optional boundary unit; `None` for exact comparison
///
/// # Returns
///
/// `true` when `date` is before `compare`.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::{umt_is_before, DateBoundaryUnit};
///
/// let early = Utc.with_ymd_and_hms(2025, 4, 15, 1, 0, 0).unwrap();
/// let late = Utc.with_ymd_and_hms(2025, 4, 15, 23, 0, 0).unwrap();
/// assert!(umt_is_before(&early, &late, None));
/// assert!(!umt_is_before(&early, &late, Some(DateBoundaryUnit::Day)));
/// ```
pub fn umt_is_before(
    date: &DateTime<Utc>,
    compare: &DateTime<Utc>,
    unit: Option<DateBoundaryUnit>,
) -> bool {
    match unit {
        Some(boundary) => umt_start_of(date, boundary) < umt_start_of(compare, boundary),
        None => date.timestamp_millis() < compare.timestamp_millis(),
    }
}
