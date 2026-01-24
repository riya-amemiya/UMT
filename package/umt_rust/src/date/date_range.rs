//! Date range generation utilities.

use chrono::{Duration, NaiveDate};

/// Generates an array containing all dates between the specified start and end dates.
///
/// # Arguments
///
/// * `start_date` - The start date of the range (inclusive).
/// * `end_date` - The end date of the range (inclusive).
///
/// # Returns
///
/// A vector of NaiveDate objects from start_date to end_date (inclusive).
/// Returns an empty vector if start_date is after end_date.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use umt_rust::date::umt_date_range;
///
/// let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
/// let dates = umt_date_range(&start, &end);
///
/// assert_eq!(dates.len(), 3);
/// assert_eq!(dates[0], NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
/// assert_eq!(dates[1], NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());
/// assert_eq!(dates[2], NaiveDate::from_ymd_opt(2025, 1, 3).unwrap());
/// ```
pub fn umt_date_range(start_date: &NaiveDate, end_date: &NaiveDate) -> Vec<NaiveDate> {
    let mut dates = Vec::new();
    let mut current_date = *start_date;

    while current_date <= *end_date {
        dates.push(current_date);
        current_date = current_date + Duration::days(1);
    }

    dates
}

/// Generates a date range from year, month, day components.
///
/// # Arguments
///
/// * `start_year` - Start year.
/// * `start_month` - Start month (1-12).
/// * `start_day` - Start day.
/// * `end_year` - End year.
/// * `end_month` - End month (1-12).
/// * `end_day` - End day.
///
/// # Returns
///
/// An Option containing a vector of NaiveDate objects, or None if dates are invalid.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use umt_rust::date::umt_date_range_ymd;
///
/// let dates = umt_date_range_ymd(2025, 1, 1, 2025, 1, 3);
/// assert!(dates.is_some());
/// assert_eq!(dates.unwrap().len(), 3);
/// ```
pub fn umt_date_range_ymd(
    start_year: i32,
    start_month: u32,
    start_day: u32,
    end_year: i32,
    end_month: u32,
    end_day: u32,
) -> Option<Vec<NaiveDate>> {
    let start = NaiveDate::from_ymd_opt(start_year, start_month, start_day)?;
    let end = NaiveDate::from_ymd_opt(end_year, end_month, end_day)?;
    Some(umt_date_range(&start, &end))
}

/// Generates an array of dates for a specific number of days from a start date.
///
/// # Arguments
///
/// * `start_date` - The start date.
/// * `days` - Number of days to include (including start date).
///
/// # Returns
///
/// A vector of NaiveDate objects.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use umt_rust::date::umt_date_range_days;
///
/// let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// let dates = umt_date_range_days(&start, 5);
///
/// assert_eq!(dates.len(), 5);
/// ```
pub fn umt_date_range_days(start_date: &NaiveDate, days: u32) -> Vec<NaiveDate> {
    if days == 0 {
        return Vec::new();
    }

    let mut dates = Vec::with_capacity(days as usize);
    let mut current_date = *start_date;

    for _ in 0..days {
        dates.push(current_date);
        current_date = current_date + Duration::days(1);
    }

    dates
}

/// Iterator for date ranges.
pub struct DateRangeIterator {
    current: NaiveDate,
    end: NaiveDate,
}

impl DateRangeIterator {
    /// Creates a new DateRangeIterator.
    pub fn new(start: NaiveDate, end: NaiveDate) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for DateRangeIterator {
    type Item = NaiveDate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.end {
            return None;
        }

        let result = self.current;
        self.current = self.current + Duration::days(1);
        Some(result)
    }
}

/// Creates an iterator over dates in a range.
///
/// This is more memory-efficient than `umt_date_range` for large ranges
/// as it doesn't allocate all dates upfront.
///
/// # Arguments
///
/// * `start_date` - The start date of the range (inclusive).
/// * `end_date` - The end date of the range (inclusive).
///
/// # Returns
///
/// A DateRangeIterator that yields dates from start to end.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use umt_rust::date::umt_date_range_iter;
///
/// let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// let end = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
///
/// let dates: Vec<_> = umt_date_range_iter(&start, &end).collect();
/// assert_eq!(dates.len(), 3);
/// ```
#[inline]
pub fn umt_date_range_iter(start_date: &NaiveDate, end_date: &NaiveDate) -> DateRangeIterator {
    DateRangeIterator::new(*start_date, *end_date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_range_basic() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        let dates = umt_date_range(&start, &end);

        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());
        assert_eq!(dates[2], NaiveDate::from_ymd_opt(2025, 1, 3).unwrap());
    }

    #[test]
    fn test_date_range_single_day() {
        let date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let dates = umt_date_range(&date, &date);

        assert_eq!(dates.len(), 1);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
    }

    #[test]
    fn test_date_range_creates_new_instances() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let dates = umt_date_range(&start, &end);

        // The returned date should be a separate instance
        assert_eq!(dates[0], start);
    }

    #[test]
    fn test_date_range_month_and_year_transitions() {
        let start = NaiveDate::from_ymd_opt(2024, 12, 30).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 2).unwrap();
        let dates = umt_date_range(&start, &end);

        assert_eq!(dates.len(), 4);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2024, 12, 30).unwrap());
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 12, 31).unwrap());
        assert_eq!(dates[2], NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert_eq!(dates[3], NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());
    }

    #[test]
    fn test_date_range_end_before_start() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let dates = umt_date_range(&start, &end);

        assert_eq!(dates.len(), 0);
    }

    #[test]
    fn test_date_range_ymd() {
        let dates = umt_date_range_ymd(2025, 1, 1, 2025, 1, 3);
        assert!(dates.is_some());
        let dates = dates.unwrap();
        assert_eq!(dates.len(), 3);
    }

    #[test]
    fn test_date_range_ymd_invalid() {
        let dates = umt_date_range_ymd(2025, 2, 30, 2025, 3, 1);
        assert!(dates.is_none());
    }

    #[test]
    fn test_date_range_days() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let dates = umt_date_range_days(&start, 5);

        assert_eq!(dates.len(), 5);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert_eq!(dates[4], NaiveDate::from_ymd_opt(2025, 1, 5).unwrap());
    }

    #[test]
    fn test_date_range_days_zero() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let dates = umt_date_range_days(&start, 0);
        assert_eq!(dates.len(), 0);
    }

    #[test]
    fn test_date_range_iterator() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();

        let dates: Vec<_> = umt_date_range_iter(&start, &end).collect();
        assert_eq!(dates.len(), 3);
        assert_eq!(dates[0], NaiveDate::from_ymd_opt(2025, 1, 1).unwrap());
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2025, 1, 2).unwrap());
        assert_eq!(dates[2], NaiveDate::from_ymd_opt(2025, 1, 3).unwrap());
    }

    #[test]
    fn test_date_range_iterator_empty() {
        let start = NaiveDate::from_ymd_opt(2025, 1, 3).unwrap();
        let end = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();

        let dates: Vec<_> = umt_date_range_iter(&start, &end).collect();
        assert_eq!(dates.len(), 0);
    }

    #[test]
    fn test_date_range_leap_year() {
        let start = NaiveDate::from_ymd_opt(2024, 2, 28).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
        let dates = umt_date_range(&start, &end);

        assert_eq!(dates.len(), 3); // Feb 28, Feb 29, Mar 1
        assert_eq!(dates[1], NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());
    }
}
