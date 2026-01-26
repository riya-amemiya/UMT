//! Day of week calculation utility.
//!
//! This module provides a function to get the day of the week for a given date.

use chrono::{Datelike, NaiveDate};

use super::now::umt_now;

/// Get the day of the week for a given date.
///
/// # Arguments
///
/// * `year` - The year (optional, defaults to current year)
/// * `month` - The month 1-12 (optional, defaults to current month)
/// * `day` - The day of month (optional, defaults to current day)
/// * `time_difference` - Time difference from UTC in hours (default: 9 for JST)
///
/// # Returns
///
/// A number representing the day of the week (0 = Sunday, 6 = Saturday)
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_day_of_week;
///
/// // Get day of week for a specific date
/// let dow = umt_day_of_week(Some(2025), Some(1), Some(1), None);
/// // January 1, 2025 is a Wednesday (3)
/// assert_eq!(dow, Some(3));
/// ```
pub fn umt_day_of_week(
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    time_difference: Option<i32>,
) -> Option<u32> {
    let tz_offset = time_difference.unwrap_or(9);
    let now = umt_now(tz_offset);

    let y = year.unwrap_or(now.year());
    let m = month.unwrap_or(now.month());
    let d = day.unwrap_or(now.day());

    let date = NaiveDate::from_ymd_opt(y, m, d)?;

    // chrono uses Monday = 0, so we need to convert to Sunday = 0
    // chrono: Mon=0, Tue=1, Wed=2, Thu=3, Fri=4, Sat=5, Sun=6
    // target: Sun=0, Mon=1, Tue=2, Wed=3, Thu=4, Fri=5, Sat=6
    let weekday = date.weekday().num_days_from_sunday();

    Some(weekday)
}

/// Get the current day of the week.
///
/// # Arguments
///
/// * `time_difference` - Time difference from UTC in hours (default: 9 for JST)
///
/// # Returns
///
/// A number representing the day of the week (0 = Sunday, 6 = Saturday)
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_today_day_of_week;
///
/// let dow = umt_today_day_of_week(None);
/// assert!(dow <= 6);
/// ```
#[inline]
pub fn umt_today_day_of_week(time_difference: Option<i32>) -> u32 {
    umt_day_of_week(None, None, None, time_difference).unwrap_or(0)
}
