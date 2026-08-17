//! Sunday-start week-of-year.

use chrono::{DateTime, Datelike, TimeZone, Utc};

use super::start_of::{DateBoundaryUnit, umt_start_of};

/// Returns the Sunday-start week number of the date's year.
///
/// Week 1 is the Sunday-start week that contains January 1.
/// The `DateTime<Utc>` fields are treated as the working wall-clock.
///
/// # Arguments
///
/// * `date` - Date to inspect
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_week_of_year;
///
/// let jan1 = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// assert_eq!(umt_week_of_year(&jan1), 1);
/// ```
pub fn umt_week_of_year(date: &DateTime<Utc>) -> i64 {
    let week_start = umt_start_of(date, DateBoundaryUnit::Week);
    let year_start = Utc
        .with_ymd_and_hms(date.year(), 1, 1, 0, 0, 0)
        .single()
        .expect("valid January 1");
    let year_start_week = umt_start_of(&year_start, DateBoundaryUnit::Week);
    let days = (week_start - year_start_week).num_days();
    days / 7 + 1
}
