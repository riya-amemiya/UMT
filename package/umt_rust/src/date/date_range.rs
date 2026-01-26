//! Date range generation utility.
//!
//! This module provides a function to generate an array of dates between two dates.

use crate::internal::datetime::{UmtDateTime, UmtDuration};

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
/// use umt_rust::date::umt_date_range;
/// use umt_rust::internal::datetime::UmtDateTime;
///
/// let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let end = UmtDateTime::from_ymd_hms(2025, 1, 3, 0, 0, 0).unwrap();
/// let dates = umt_date_range(start, end);
/// assert_eq!(dates.len(), 3);
/// ```
pub fn umt_date_range(start_date: UmtDateTime, end_date: UmtDateTime) -> Vec<UmtDateTime> {
    let mut dates: Vec<UmtDateTime> = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        dates.push(current_date);
        current_date = current_date + UmtDuration::days(1);
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
/// use umt_rust::date::umt_date_range_with_step;
/// use umt_rust::internal::datetime::UmtDateTime;
///
/// let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let end = UmtDateTime::from_ymd_hms(2025, 1, 10, 0, 0, 0).unwrap();
/// let dates = umt_date_range_with_step(start, end, 2);
/// assert_eq!(dates.len(), 5); // 1, 3, 5, 7, 9
/// ```
pub fn umt_date_range_with_step(
    start_date: UmtDateTime,
    end_date: UmtDateTime,
    step_days: i64,
) -> Vec<UmtDateTime> {
    if step_days <= 0 {
        return Vec::new();
    }

    let mut dates: Vec<UmtDateTime> = Vec::new();
    let mut current_date = start_date;

    while current_date <= end_date {
        dates.push(current_date);
        current_date = current_date + UmtDuration::days(step_days);
    }

    dates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_three_days() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = UmtDateTime::from_ymd_hms(2025, 1, 3, 0, 0, 0).unwrap();
        let dates = umt_date_range(start, end);

        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0].format_str("%Y-%m-%d"), "2025-01-01");
        assert_eq!(dates[1].format_str("%Y-%m-%d"), "2025-01-02");
        assert_eq!(dates[2].format_str("%Y-%m-%d"), "2025-01-03");
    }

    #[test]
    fn test_date_range_single_day() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let dates = umt_date_range(start, start);

        assert_eq!(dates.len(), 1);
        assert_eq!(dates[0], start);
    }

    #[test]
    fn test_date_range_end_before_start() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 5, 0, 0, 0).unwrap();
        let end = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let dates = umt_date_range(start, end);

        assert!(dates.is_empty());
    }

    #[test]
    fn test_date_range_month_boundary() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 30, 0, 0, 0).unwrap();
        let end = UmtDateTime::from_ymd_hms(2025, 2, 2, 0, 0, 0).unwrap();
        let dates = umt_date_range(start, end);

        assert_eq!(dates.len(), 4);
        assert_eq!(dates[0].format_str("%Y-%m-%d"), "2025-01-30");
        assert_eq!(dates[1].format_str("%Y-%m-%d"), "2025-01-31");
        assert_eq!(dates[2].format_str("%Y-%m-%d"), "2025-02-01");
        assert_eq!(dates[3].format_str("%Y-%m-%d"), "2025-02-02");
    }

    #[test]
    fn test_date_range_with_step() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = UmtDateTime::from_ymd_hms(2025, 1, 10, 0, 0, 0).unwrap();
        let dates = umt_date_range_with_step(start, end, 2);

        assert_eq!(dates.len(), 5);
        assert_eq!(dates[0].format_str("%Y-%m-%d"), "2025-01-01");
        assert_eq!(dates[1].format_str("%Y-%m-%d"), "2025-01-03");
        assert_eq!(dates[2].format_str("%Y-%m-%d"), "2025-01-05");
        assert_eq!(dates[3].format_str("%Y-%m-%d"), "2025-01-07");
        assert_eq!(dates[4].format_str("%Y-%m-%d"), "2025-01-09");
    }

    #[test]
    fn test_date_range_with_step_zero() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = UmtDateTime::from_ymd_hms(2025, 1, 10, 0, 0, 0).unwrap();
        let dates = umt_date_range_with_step(start, end, 0);

        assert!(dates.is_empty());
    }

    #[test]
    fn test_date_range_with_negative_step() {
        let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
        let end = UmtDateTime::from_ymd_hms(2025, 1, 10, 0, 0, 0).unwrap();
        let dates = umt_date_range_with_step(start, end, -1);

        assert!(dates.is_empty());
    }
}
