//! Date range generation utility.
//!
//! This module provides a function to generate an array of dates between two dates.

use chrono::{DateTime, Duration, Utc};

/// Generate a vector containing all dates between the specified start and end dates.
///
/// # Arguments
///
/// * `start_date` - The start date of the range
/// * `end_date` - The end date of the range
///
/// # Returns
///
/// A vector of DateTime objects from start_date to end_date (inclusive)
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_date_range;
///
/// let start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let end = Utc.with_ymd_and_hms(2025, 1, 3, 0, 0, 0).unwrap();
/// let dates = umt_date_range(start, end);
/// assert_eq!(dates.len(), 3);
/// ```
pub fn umt_date_range(start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Vec<DateTime<Utc>> {
    let mut dates: Vec<DateTime<Utc>> = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        dates.push(current_date);
        current_date += Duration::days(1);
    }

    dates
}

/// Generate a date range with a custom step in days.
///
/// # Arguments
///
/// * `start_date` - The start date of the range
/// * `end_date` - The end date of the range
/// * `step_days` - Number of days between each date
///
/// # Returns
///
/// A vector of DateTime objects
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_date_range_with_step;
///
/// let start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let end = Utc.with_ymd_and_hms(2025, 1, 10, 0, 0, 0).unwrap();
/// let dates = umt_date_range_with_step(start, end, 2);
/// assert_eq!(dates.len(), 5); // 1, 3, 5, 7, 9
/// ```
pub fn umt_date_range_with_step(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    step_days: i64,
) -> Vec<DateTime<Utc>> {
    if step_days <= 0 {
        return Vec::new();
    }

    let mut dates: Vec<DateTime<Utc>> = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        dates.push(current_date);
        current_date += Duration::days(step_days);
    }

    dates
}
