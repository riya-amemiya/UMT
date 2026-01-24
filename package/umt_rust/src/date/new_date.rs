//! Date creation utilities.
//!
//! This module provides functions to create Date objects from various inputs.

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};

/// Create a new DateTime from numeric values.
///
/// # Arguments
///
/// * `year` - The year
/// * `month` - The month (1-12)
/// * `day` - The day of the month (1-31)
/// * `hours` - Hours (0-23), defaults to 0
/// * `minutes` - Minutes (0-59), defaults to 0
/// * `seconds` - Seconds (0-59), defaults to 0
/// * `milliseconds` - Milliseconds (0-999), defaults to 0
///
/// # Returns
///
/// `Option<DateTime<Utc>>` - The created DateTime or None if invalid
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_new_date_int;
///
/// let date = umt_new_date_int(2021, 1, 1, None, None, None, None);
/// assert!(date.is_some());
/// ```
pub fn umt_new_date_int(
    year: i32,
    month: u32,
    day: u32,
    hours: Option<u32>,
    minutes: Option<u32>,
    seconds: Option<u32>,
    milliseconds: Option<u32>,
) -> Option<DateTime<Utc>> {
    let h = hours.unwrap_or(0);
    let m = minutes.unwrap_or(0);
    let s = seconds.unwrap_or(0);
    let ms = milliseconds.unwrap_or(0);

    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    let time = NaiveTime::from_hms_milli_opt(h, m, s, ms)?;
    let naive_datetime = NaiveDateTime::new(date, time);

    Some(Utc.from_utc_datetime(&naive_datetime))
}

/// Create a new DateTime from a date string.
///
/// # Arguments
///
/// * `date_str` - Date string in format "YYYY-MM-DD"
/// * `hours` - Hours string in "HH" format (00-23), defaults to "00"
/// * `minutes` - Minutes string in "mm" format (00-59), defaults to "00"
/// * `seconds` - Seconds string in "ss" format (00-59), defaults to "00"
/// * `milliseconds` - Milliseconds string in "mmm" format (000-999), defaults to "000"
/// * `time_difference` - Timezone offset hours, defaults to 0
///
/// # Returns
///
/// `Option<DateTime<Utc>>` - The created DateTime or None if invalid
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_new_date_string;
///
/// let date = umt_new_date_string("2021-01-01", None, None, None, None, None);
/// assert!(date.is_some());
/// ```
pub fn umt_new_date_string(
    date_str: &str,
    hours: Option<&str>,
    minutes: Option<&str>,
    seconds: Option<&str>,
    milliseconds: Option<&str>,
    time_difference: Option<i32>,
) -> Option<DateTime<Utc>> {
    let h: u32 = hours.unwrap_or("00").parse().ok()?;
    let m: u32 = minutes.unwrap_or("00").parse().ok()?;
    let s: u32 = seconds.unwrap_or("00").parse().ok()?;
    let ms: u32 = milliseconds.unwrap_or("000").parse().ok()?;
    let offset_hours = time_difference.unwrap_or(0);

    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;
    let time = NaiveTime::from_hms_milli_opt(h, m, s, ms)?;
    let naive_datetime = NaiveDateTime::new(date, time);

    // Apply timezone offset
    let utc_datetime = Utc.from_utc_datetime(&naive_datetime);
    Some(utc_datetime - chrono::Duration::hours(offset_hours as i64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_date_int_basic() {
        let date = umt_new_date_int(2021, 1, 1, None, None, None, None);
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2021-01-01");
    }

    #[test]
    fn test_new_date_int_with_time() {
        let date = umt_new_date_int(2021, 6, 15, Some(14), Some(30), Some(45), Some(500));
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2021-06-15 14:30:45");
    }

    #[test]
    fn test_new_date_int_invalid_month() {
        let date = umt_new_date_int(2021, 13, 1, None, None, None, None);
        assert!(date.is_none());
    }

    #[test]
    fn test_new_date_int_invalid_day() {
        let date = umt_new_date_int(2021, 2, 30, None, None, None, None);
        assert!(date.is_none());
    }

    #[test]
    fn test_new_date_string_basic() {
        let date = umt_new_date_string("2021-01-01", None, None, None, None, None);
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2021-01-01");
    }

    #[test]
    fn test_new_date_string_with_time() {
        let date = umt_new_date_string("2021-06-15", Some("14"), Some("30"), Some("45"), Some("500"), None);
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%H:%M:%S").to_string(), "14:30:45");
    }

    #[test]
    fn test_new_date_string_invalid_format() {
        let date = umt_new_date_string("01-01-2021", None, None, None, None, None);
        assert!(date.is_none());
    }

    #[test]
    fn test_new_date_string_with_timezone() {
        let date_utc = umt_new_date_string("2021-01-01", Some("12"), None, None, None, Some(0));
        let date_jst = umt_new_date_string("2021-01-01", Some("12"), None, None, None, Some(9));
        assert!(date_utc.is_some());
        assert!(date_jst.is_some());
        let diff = date_utc.unwrap().signed_duration_since(date_jst.unwrap());
        assert_eq!(diff.num_hours(), -9);
    }
}
