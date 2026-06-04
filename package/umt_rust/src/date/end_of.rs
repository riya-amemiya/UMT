//! End-of-unit date boundary calculation.

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};

use super::start_of::DateBoundaryUnit;

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

/// Returns the last day of the given (1-based) month.
fn last_day_of_month(year: i32, month: u32) -> u32 {
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let first_of_next =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("valid first-of-month date");
    first_of_next.pred_opt().expect("valid previous day").day()
}

/// Returns a new `DateTime<Utc>` set to the end of the given unit.
///
/// Week ends on Saturday at 23:59:59.999. The `DateTime<Utc>` fields are
/// treated as the working wall-clock, matching the local-time semantics of
/// the TypeScript reference.
///
/// # Arguments
///
/// * `date` - Base date
/// * `unit` - Boundary unit
///
/// # Returns
///
/// A new `DateTime<Utc>` at the end of the specified unit.
///
/// # Examples
///
/// ```
/// use chrono::{Datelike, Timelike, TimeZone, Utc};
/// use umt_rust::date::{umt_end_of, DateBoundaryUnit};
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 15, 10, 30, 0).unwrap();
/// let result = umt_end_of(&date, DateBoundaryUnit::Day);
/// assert_eq!(result.hour(), 23);
/// assert_eq!(result.minute(), 59);
/// assert_eq!(result.second(), 59);
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
/// let result = umt_end_of(&date, DateBoundaryUnit::Month);
/// assert_eq!(result.day(), 30);
/// ```
pub fn umt_end_of(date: &DateTime<Utc>, unit: DateBoundaryUnit) -> DateTime<Utc> {
    let year = date.year();
    let month = date.month();
    let day = date.day();
    let hour = date.hour();
    let minute = date.minute();
    let second = date.second();

    match unit {
        DateBoundaryUnit::Second => build(year, month, day, hour, minute, second, 999),
        DateBoundaryUnit::Minute => build(year, month, day, hour, minute, 59, 999),
        DateBoundaryUnit::Hour => build(year, month, day, hour, 59, 59, 999),
        DateBoundaryUnit::Day => build(year, month, day, 23, 59, 59, 999),
        DateBoundaryUnit::Week => {
            let day_of_week = date.weekday().num_days_from_sunday() as i64;
            build(year, month, day, 23, 59, 59, 999) + Duration::days(6 - day_of_week)
        }
        DateBoundaryUnit::Month => {
            let last = last_day_of_month(year, month);
            build(year, month, last, 23, 59, 59, 999)
        }
        DateBoundaryUnit::Quarter => {
            // 0-based quarter-end month, then convert to 1-based.
            let quarter_end_month0 = ((month - 1) / 3) * 3 + 2;
            let quarter_end_month = quarter_end_month0 + 1;
            let last = last_day_of_month(year, quarter_end_month);
            build(year, quarter_end_month, last, 23, 59, 59, 999)
        }
        DateBoundaryUnit::Year => build(year, 12, 31, 23, 59, 59, 999),
    }
}
