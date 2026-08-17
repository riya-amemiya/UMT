//! Calendar quarter for a date.

use chrono::{DateTime, Datelike, Utc};

/// Returns the calendar quarter (1-4) for the date's month.
///
/// Matches `umt_start_of(..., Quarter)`: Jan-Mar=1, Apr-Jun=2, Jul-Sep=3,
/// Oct-Dec=4.
///
/// # Arguments
///
/// * `date` - Date to inspect
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_get_quarter;
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 15, 0, 0, 0).unwrap();
/// assert_eq!(umt_get_quarter(&date), 2);
/// ```
pub fn umt_get_quarter(date: &DateTime<Utc>) -> u32 {
    (date.month() - 1) / 3 + 1
}
