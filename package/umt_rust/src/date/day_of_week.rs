//! Day of week calculation utility.
//!
//! This module provides a function to get the day of the week for a given date.

use crate::internal::datetime::UmtNaiveDate;

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

    let date = UmtNaiveDate::from_ymd_opt(y, m, d)?;

    // Sunday = 0, Monday = 1, ..., Saturday = 6
    let weekday = date.weekday_num_from_sunday();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_of_week_known_date() {
        // January 1, 2025 is a Wednesday
        let dow = umt_day_of_week(Some(2025), Some(1), Some(1), Some(0));
        assert_eq!(dow, Some(3)); // Wednesday = 3
    }

    #[test]
    fn test_day_of_week_sunday() {
        // January 5, 2025 is a Sunday
        let dow = umt_day_of_week(Some(2025), Some(1), Some(5), Some(0));
        assert_eq!(dow, Some(0)); // Sunday = 0
    }

    #[test]
    fn test_day_of_week_saturday() {
        // January 4, 2025 is a Saturday
        let dow = umt_day_of_week(Some(2025), Some(1), Some(4), Some(0));
        assert_eq!(dow, Some(6)); // Saturday = 6
    }

    #[test]
    fn test_day_of_week_invalid_date() {
        let dow = umt_day_of_week(Some(2025), Some(2), Some(30), Some(0));
        assert!(dow.is_none());
    }

    #[test]
    fn test_today_day_of_week() {
        let dow = umt_today_day_of_week(Some(0));
        assert!(dow <= 6);
    }

    #[test]
    fn test_day_of_week_all_days() {
        // Week of Jan 5-11, 2025
        // Sun=5, Mon=6, Tue=7, Wed=8, Thu=9, Fri=10, Sat=11
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(5), Some(0)),
            Some(0)
        ); // Sun
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(6), Some(0)),
            Some(1)
        ); // Mon
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(7), Some(0)),
            Some(2)
        ); // Tue
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(8), Some(0)),
            Some(3)
        ); // Wed
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(9), Some(0)),
            Some(4)
        ); // Thu
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(10), Some(0)),
            Some(5)
        ); // Fri
        assert_eq!(
            umt_day_of_week(Some(2025), Some(1), Some(11), Some(0)),
            Some(6)
        ); // Sat
    }
}
