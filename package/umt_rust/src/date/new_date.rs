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
