//! Day of week calculation utilities.

use crate::date::now::umt_now;
use chrono::{Datelike, NaiveDate, Weekday};

/// Gets the day of the week for a specific date.
///
/// # Arguments
///
/// * `year` - The year (optional, defaults to current year).
/// * `month` - The month 1-12 (optional, defaults to current month).
/// * `day` - The day of month (optional, defaults to current day).
/// * `time_difference` - Time difference from UTC in hours (default: 9 for JST).
///
/// # Returns
///
/// A number representing the day of the week (0 = Sunday, 6 = Saturday).
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_day_of_week;
///
/// // January 1, 2020 was a Wednesday (3)
/// let day = umt_day_of_week(Some(2020), Some(1), Some(1), 9);
/// assert_eq!(day, 3);
/// ```
pub fn umt_day_of_week(
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    time_difference: i32,
) -> u32 {
    let now_time = umt_now(time_difference);

    let y = year.unwrap_or(now_time.year());
    let m = month.unwrap_or(now_time.month());
    let d = day.unwrap_or(now_time.day());

    let date = NaiveDate::from_ymd_opt(y, m, d).unwrap_or_else(|| {
        NaiveDate::from_ymd_opt(now_time.year(), now_time.month(), now_time.day()).unwrap()
    });

    // Convert chrono's Weekday (Mon=0 to Sun=6) to JavaScript convention (Sun=0 to Sat=6)
    weekday_to_js(date.weekday())
}

/// Gets the day of the week for a specific date using default timezone (JST/UTC+9).
///
/// # Arguments
///
/// * `year` - The year (optional, defaults to current year).
/// * `month` - The month 1-12 (optional, defaults to current month).
/// * `day` - The day of month (optional, defaults to current day).
///
/// # Returns
///
/// A number representing the day of the week (0 = Sunday, 6 = Saturday).
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_day_of_week_jst;
///
/// // January 1, 2020 was a Wednesday (3)
/// let day = umt_day_of_week_jst(Some(2020), Some(1), Some(1));
/// assert_eq!(day, 3);
/// ```
#[inline]
pub fn umt_day_of_week_jst(year: Option<i32>, month: Option<u32>, day: Option<u32>) -> u32 {
    umt_day_of_week(year, month, day, 9)
}

/// Gets the current day of the week.
///
/// # Arguments
///
/// * `time_difference` - Time difference from UTC in hours (default: 9 for JST).
///
/// # Returns
///
/// A number representing the day of the week (0 = Sunday, 6 = Saturday).
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_day_of_week_now;
///
/// let day = umt_day_of_week_now(9);
/// assert!(day <= 6);
/// ```
#[inline]
pub fn umt_day_of_week_now(time_difference: i32) -> u32 {
    umt_day_of_week(None, None, None, time_difference)
}

/// Gets the day of the week from a NaiveDate.
///
/// # Arguments
///
/// * `date` - The NaiveDate to get day of week from.
///
/// # Returns
///
/// A number representing the day of the week (0 = Sunday, 6 = Saturday).
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use umt_rust::date::umt_day_of_week_from_date;
///
/// let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
/// let day = umt_day_of_week_from_date(&date);
/// assert_eq!(day, 3); // Wednesday
/// ```
#[inline]
pub fn umt_day_of_week_from_date(date: &NaiveDate) -> u32 {
    weekday_to_js(date.weekday())
}

/// Converts chrono's Weekday to JavaScript-style day number.
///
/// chrono uses Mon=0 to Sun=6, JavaScript uses Sun=0 to Sat=6.
#[inline]
fn weekday_to_js(weekday: Weekday) -> u32 {
    match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_of_week_known_dates() {
        // January 1, 2020 was a Wednesday
        assert_eq!(umt_day_of_week(Some(2020), Some(1), Some(1), 9), 3);

        // January 2, 2020 was a Thursday
        assert_eq!(umt_day_of_week(Some(2020), Some(1), Some(2), 9), 4);
    }

    #[test]
    fn test_day_of_week_jst_known_dates() {
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(1)), 3);
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(2)), 4);
    }

    #[test]
    fn test_day_of_week_defaults_current_date_parts() {
        let now = umt_now(9);

        // When only year is missing
        let with_month_day = umt_day_of_week_jst(None, Some(1), Some(2));
        let expected_date = NaiveDate::from_ymd_opt(now.year(), 1, 2);
        if let Some(date) = expected_date {
            assert_eq!(with_month_day, weekday_to_js(date.weekday()));
        }
    }

    #[test]
    fn test_day_of_week_now_returns_valid() {
        let day = umt_day_of_week_now(9);
        assert!(day <= 6);
    }

    #[test]
    fn test_day_of_week_from_date() {
        let date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert_eq!(umt_day_of_week_from_date(&date), 3);

        // Sunday
        let sunday = NaiveDate::from_ymd_opt(2020, 1, 5).unwrap();
        assert_eq!(umt_day_of_week_from_date(&sunday), 0);

        // Saturday
        let saturday = NaiveDate::from_ymd_opt(2020, 1, 4).unwrap();
        assert_eq!(umt_day_of_week_from_date(&saturday), 6);
    }

    #[test]
    fn test_weekday_to_js_conversion() {
        assert_eq!(weekday_to_js(Weekday::Sun), 0);
        assert_eq!(weekday_to_js(Weekday::Mon), 1);
        assert_eq!(weekday_to_js(Weekday::Tue), 2);
        assert_eq!(weekday_to_js(Weekday::Wed), 3);
        assert_eq!(weekday_to_js(Weekday::Thu), 4);
        assert_eq!(weekday_to_js(Weekday::Fri), 5);
        assert_eq!(weekday_to_js(Weekday::Sat), 6);
    }

    #[test]
    fn test_day_of_week_all_days() {
        // Week of Jan 5-11, 2020
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(5)), 0); // Sunday
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(6)), 1); // Monday
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(7)), 2); // Tuesday
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(8)), 3); // Wednesday
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(9)), 4); // Thursday
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(10)), 5); // Friday
        assert_eq!(umt_day_of_week_jst(Some(2020), Some(1), Some(11)), 6); // Saturday
    }
}
