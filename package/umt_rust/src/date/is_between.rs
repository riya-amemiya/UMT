//! Range comparison for dates.

use chrono::{DateTime, Utc};

use super::date_inclusivity::DateInclusivity;
use super::start_of::{DateBoundaryUnit, umt_start_of};

/// Returns `true` when `date` lies between `start` and `end`.
///
/// Without a unit, compares exact timestamps. With a unit, all three dates
/// are truncated with [`umt_start_of`] first. Ranges are not swapped.
///
/// # Arguments
///
/// * `date` - Date to test
/// * `start` - Range start
/// * `end` - Range end
/// * `unit` - Optional boundary unit
/// * `inclusivity` - Bound inclusivity
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::{umt_is_between, DateInclusivity};
///
/// let start = Utc.with_ymd_and_hms(2025, 4, 10, 10, 0, 0).unwrap();
/// let mid = Utc.with_ymd_and_hms(2025, 4, 15, 12, 0, 0).unwrap();
/// let end = Utc.with_ymd_and_hms(2025, 4, 20, 18, 0, 0).unwrap();
/// assert!(umt_is_between(&mid, &start, &end, None, DateInclusivity::Exclusive));
/// ```
pub fn umt_is_between(
    date: &DateTime<Utc>,
    start: &DateTime<Utc>,
    end: &DateTime<Utc>,
    unit: Option<DateBoundaryUnit>,
    inclusivity: DateInclusivity,
) -> bool {
    let value = match unit {
        Some(boundary) => umt_start_of(date, boundary).timestamp_millis(),
        None => date.timestamp_millis(),
    };
    let from = match unit {
        Some(boundary) => umt_start_of(start, boundary).timestamp_millis(),
        None => start.timestamp_millis(),
    };
    let to = match unit {
        Some(boundary) => umt_start_of(end, boundary).timestamp_millis(),
        None => end.timestamp_millis(),
    };
    let after_start = if inclusivity.include_start() {
        value >= from
    } else {
        value > from
    };
    let before_end = if inclusivity.include_end() {
        value <= to
    } else {
        value < to
    };
    after_start && before_end
}
