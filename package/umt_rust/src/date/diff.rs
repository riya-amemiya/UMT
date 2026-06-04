//! Difference between two dates in a given unit.

use chrono::{DateTime, Datelike, Timelike, Utc};

use crate::consts::clock::{ONE_HOUR_MS, ONE_MINUTE_MS, ONE_SECOND_MS};

use super::ms_by_unit::{DurationUnit, umt_ms_by_unit};

/// Returns the difference between two dates in the given unit, truncated toward zero.
///
/// Calendar-aware for `Month` (months) and `Y` (years). The `DateTime<Utc>`
/// fields are treated as the working wall-clock, matching the local-time
/// semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `left` - End date
/// * `right` - Start date
/// * `unit` - Unit of the result
///
/// # Returns
///
/// The difference, truncated toward zero.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::{umt_diff, DurationUnit};
///
/// let left = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
/// let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// assert_eq!(umt_diff(&left, &right, DurationUnit::D), 364);
///
/// let left = Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap();
/// let right = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// assert_eq!(umt_diff(&left, &right, DurationUnit::Y), 1);
/// ```
pub fn umt_diff(left: &DateTime<Utc>, right: &DateTime<Utc>, unit: DurationUnit) -> i64 {
    if let Some(ms) = umt_ms_by_unit(unit) {
        let delta_ms = left.timestamp_millis() - right.timestamp_millis();
        // Truncate toward zero (Rust integer division already truncates toward zero).
        return delta_ms / ms as i64;
    }

    let years_delta = (left.year() - right.year()) as i64;
    let months_delta = (left.month() as i64) - (right.month() as i64);
    let day_delta = (left.day() as i64) - (right.day() as i64);

    let one_hour = ONE_HOUR_MS as i64;
    let one_minute = ONE_MINUTE_MS as i64;
    let one_second = ONE_SECOND_MS as i64;

    let left_sub = left.hour() as i64 * one_hour
        + left.minute() as i64 * one_minute
        + left.second() as i64 * one_second
        + (left.nanosecond() / 1_000_000) as i64;
    let right_sub = right.hour() as i64 * one_hour
        + right.minute() as i64 * one_minute
        + right.second() as i64 * one_second
        + (right.nanosecond() / 1_000_000) as i64;
    let sub_day_delta = left_sub - right_sub;

    let mut calendar_months = years_delta * 12 + months_delta;
    if calendar_months > 0 && (day_delta < 0 || (day_delta == 0 && sub_day_delta < 0)) {
        calendar_months -= 1;
    } else if calendar_months < 0 && (day_delta > 0 || (day_delta == 0 && sub_day_delta > 0)) {
        calendar_months += 1;
    }

    if unit == DurationUnit::Month {
        return calendar_months;
    }
    // Truncate toward zero.
    calendar_months / 12
}
