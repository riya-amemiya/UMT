//! Granular equality comparison for dates.

use chrono::{DateTime, Utc};

use super::start_of::{DateBoundaryUnit, umt_start_of};

/// Returns `true` when two dates are the same at the given unit granularity.
///
/// Without a unit, compares exact timestamps (millisecond precision). With a
/// unit, both dates are truncated with [`umt_start_of`] before comparison.
/// Week boundaries follow the Sunday-start week, matching [`umt_start_of`].
///
/// # Arguments
///
/// * `left` - First date
/// * `right` - Second date
/// * `unit` - Optional boundary unit; `None` for exact equality
///
/// # Returns
///
/// `true` when the dates match at the given granularity.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::{umt_is_same, DateBoundaryUnit};
///
/// let a = Utc.with_ymd_and_hms(2025, 4, 15, 1, 0, 0).unwrap();
/// let b = Utc.with_ymd_and_hms(2025, 4, 15, 23, 0, 0).unwrap();
/// assert!(umt_is_same(&a, &b, Some(DateBoundaryUnit::Day)));
/// assert!(!umt_is_same(&a, &b, None));
/// ```
pub fn umt_is_same(
    left: &DateTime<Utc>,
    right: &DateTime<Utc>,
    unit: Option<DateBoundaryUnit>,
) -> bool {
    match unit {
        Some(boundary) => umt_start_of(left, boundary) == umt_start_of(right, boundary),
        None => left.timestamp_millis() == right.timestamp_millis(),
    }
}
