//! Start-of-unit date boundary calculation.

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};

/// Boundary unit for [`umt_start_of`](super::umt_start_of) and
/// [`umt_end_of`](super::umt_end_of).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateBoundaryUnit {
    /// Second boundary.
    Second,
    /// Minute boundary.
    Minute,
    /// Hour boundary.
    Hour,
    /// Day boundary.
    Day,
    /// Week boundary (week starts on Sunday).
    Week,
    /// Month boundary.
    Month,
    /// Quarter boundary.
    Quarter,
    /// Year boundary.
    Year,
}

/// Builds a `DateTime<Utc>` from explicit wall-clock fields.
fn build(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    milli: u32,
) -> DateTime<Utc> {
    let date = NaiveDate::from_ymd_opt(year, month, day).expect("valid date");
    let dt = date
        .and_hms_milli_opt(hour, minute, second, milli)
        .expect("valid time-of-day");
    Utc.from_utc_datetime(&dt)
}

/// Returns a new `DateTime<Utc>` set to the start of the given unit.
///
/// Week start is Sunday (day 0). The `DateTime<Utc>` fields are treated as
/// the working wall-clock, matching the local-time semantics of the
/// TypeScript reference.
///
/// # Arguments
///
/// * `date` - Base date
/// * `unit` - Boundary unit
///
/// # Returns
///
/// A new `DateTime<Utc>` at the start of the specified unit.
///
/// # Examples
///
/// ```
/// use chrono::{Datelike, Timelike, TimeZone, Utc};
/// use umt_rust::date::{umt_start_of, DateBoundaryUnit};
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 0).unwrap();
/// let result = umt_start_of(&date, DateBoundaryUnit::Day);
/// assert_eq!(result.hour(), 0);
/// assert_eq!(result.minute(), 0);
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
/// let result = umt_start_of(&date, DateBoundaryUnit::Month);
/// assert_eq!(result.day(), 1);
/// ```
pub fn umt_start_of(date: &DateTime<Utc>, unit: DateBoundaryUnit) -> DateTime<Utc> {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let hour = date.hour();
    let minute = date.minute();
    let second = date.second();

    match unit {
        DateBoundaryUnit::Second => build(year, month, day, hour, minute, second, 0),
        DateBoundaryUnit::Minute => build(year, month, day, hour, minute, 0, 0),
        DateBoundaryUnit::Hour => build(year, month, day, hour, 0, 0, 0),
        DateBoundaryUnit::Day => build(year, month, day, 0, 0, 0, 0),
        DateBoundaryUnit::Week => {
            let day_of_week = date.weekday().num_days_from_sunday() as i64;
            build(year, month, day, 0, 0, 0, 0) - Duration::days(day_of_week)
        }
        DateBoundaryUnit::Month => build(year, month, 1, 0, 0, 0, 0),
        DateBoundaryUnit::Quarter => {
            let quarter_start_month = ((month - 1) / 3) * 3 + 1;
            build(year, quarter_start_month, 1, 0, 0, 0, 0)
        }
        DateBoundaryUnit::Year => build(year, 1, 1, 0, 0, 0, 0),
    }
}
