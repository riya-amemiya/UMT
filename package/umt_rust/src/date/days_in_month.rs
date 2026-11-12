//! Number of days in a month.

use chrono::{DateTime, Datelike, Utc};

use super::is_leap_year::umt_is_leap_year;

/// Returns the number of days in the month of the given date.
///
/// Leap years follow the Gregorian calendar (February has 29 days). The
/// `DateTime<Utc>` fields are treated as the working wall-clock, matching the
/// local-time semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `date` - Any date within the month
///
/// # Returns
///
/// Number of days in that month (28-31).
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_days_in_month;
///
/// let january = Utc.with_ymd_and_hms(2025, 1, 15, 0, 0, 0).unwrap();
/// assert_eq!(umt_days_in_month(&january), 31);
/// let leap_february = Utc.with_ymd_and_hms(2024, 2, 1, 0, 0, 0).unwrap();
/// assert_eq!(umt_days_in_month(&leap_february), 29);
/// ```
pub fn umt_days_in_month(date: &DateTime<Utc>) -> u32 {
    days_in_year_month(date.year(), date.month())
}

fn days_in_year_month(year: i32, month: u32) -> u32 {
    match month {
        2 if umt_is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}
