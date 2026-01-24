//! Date creation utilities.

use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

/// Creates a new DateTime from numeric values.
///
/// # Arguments
///
/// * `year` - The year.
/// * `month` - The month (1-12).
/// * `day` - The day of the month.
/// * `hours` - Hours (0-23). Defaults to local timezone offset hours if None.
/// * `minutes` - Minutes (0-59). Defaults to 0 if None.
/// * `seconds` - Seconds (0-59). Defaults to 0 if None.
/// * `milliseconds` - Milliseconds (0-999). Defaults to 0 if None.
///
/// # Returns
///
/// An Option containing the DateTime, or None if the date is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_new_date_int;
///
/// // Creates date for January 1, 2025
/// let date = umt_new_date_int(2025, 1, 1, None, None, None, None);
/// assert!(date.is_some());
///
/// // Creates date with specific time
/// let date = umt_new_date_int(2025, 1, 1, Some(10), Some(30), Some(45), Some(500));
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
) -> Option<NaiveDateTime> {
    let date = NaiveDate::from_ymd_opt(year, month, day)?;

    // Get default hours from local timezone offset if not provided
    let default_hours = {
        let local_now = Local::now();
        let offset_seconds = local_now.offset().local_minus_utc();
        (offset_seconds / 3600).unsigned_abs()
    };

    let time = NaiveTime::from_hms_milli_opt(
        hours.unwrap_or(default_hours),
        minutes.unwrap_or(0),
        seconds.unwrap_or(0),
        milliseconds.unwrap_or(0),
    )?;

    Some(NaiveDateTime::new(date, time))
}

/// Creates a new DateTime from numeric values with all parameters required.
///
/// # Arguments
///
/// * `year` - The year.
/// * `month` - The month (1-12).
/// * `day` - The day of the month.
/// * `hours` - Hours (0-23).
/// * `minutes` - Minutes (0-59).
/// * `seconds` - Seconds (0-59).
/// * `milliseconds` - Milliseconds (0-999).
///
/// # Returns
///
/// An Option containing the NaiveDateTime, or None if the date/time is invalid.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_new_date_int_full;
///
/// let date = umt_new_date_int_full(2025, 1, 1, 10, 30, 45, 500);
/// assert!(date.is_some());
/// let dt = date.unwrap();
/// assert_eq!(dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string(), "2025-01-01 10:30:45.500");
/// ```
pub fn umt_new_date_int_full(
    year: i32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
    milliseconds: u32,
) -> Option<NaiveDateTime> {
    let date = NaiveDate::from_ymd_opt(year, month, day)?;
    let time = NaiveTime::from_hms_milli_opt(hours, minutes, seconds, milliseconds)?;
    Some(NaiveDateTime::new(date, time))
}

/// Creates a new DateTime from a date string and time components.
///
/// # Arguments
///
/// * `date_str` - Date string in format "YYYY-MM-DD".
/// * `hours` - Hours in format "HH" (00-23). Defaults to "00".
/// * `minutes` - Minutes in format "mm" (00-59). Defaults to "00".
/// * `seconds` - Seconds in format "ss" (00-59). Defaults to "00".
/// * `milliseconds` - Milliseconds in format "mmm" (000-999). Defaults to "000".
/// * `time_difference` - Timezone offset in hours format (e.g., "09" for UTC+9). Defaults to "00".
///
/// # Returns
///
/// An Option containing the DateTime with timezone, or None if parsing fails.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_new_date_string;
///
/// // Creates date for January 1, 2025 00:00:00 UTC
/// let date = umt_new_date_string("2025-01-01", None, None, None, None, None);
/// assert!(date.is_some());
///
/// // Creates date with specific time and timezone
/// let date = umt_new_date_string(
///     "2025-01-01",
///     Some("10"),
///     Some("30"),
///     Some("45"),
///     Some("500"),
///     Some("09")
/// );
/// assert!(date.is_some());
/// ```
pub fn umt_new_date_string(
    date_str: &str,
    hours: Option<&str>,
    minutes: Option<&str>,
    seconds: Option<&str>,
    milliseconds: Option<&str>,
    time_difference: Option<&str>,
) -> Option<DateTime<FixedOffset>> {
    let hours = hours.unwrap_or("00");
    let minutes = minutes.unwrap_or("00");
    let seconds = seconds.unwrap_or("00");
    let milliseconds = milliseconds.unwrap_or("000");
    let time_difference = time_difference.unwrap_or("00");

    // Parse the date string
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok()?;

    // Parse time components
    let h: u32 = hours.parse().ok()?;
    let m: u32 = minutes.parse().ok()?;
    let s: u32 = seconds.parse().ok()?;
    let ms: u32 = milliseconds.parse().ok()?;

    let time = NaiveTime::from_hms_milli_opt(h, m, s, ms)?;
    let naive_dt = NaiveDateTime::new(date, time);

    // Parse timezone offset
    let tz_hours: i32 = time_difference.parse().ok()?;
    let offset = FixedOffset::east_opt(tz_hours * 3600)?;

    Some(offset.from_local_datetime(&naive_dt).single()?)
}

/// Creates a new DateTime from a full ISO 8601 string.
///
/// # Arguments
///
/// * `datetime_str` - DateTime string in ISO 8601 format (e.g., "2025-01-01T10:30:45+09:00").
///
/// # Returns
///
/// An Option containing the DateTime with timezone, or None if parsing fails.
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_new_date_from_iso;
///
/// let date = umt_new_date_from_iso("2025-01-01T10:30:45+09:00");
/// assert!(date.is_some());
/// ```
pub fn umt_new_date_from_iso(datetime_str: &str) -> Option<DateTime<FixedOffset>> {
    DateTime::parse_from_rfc3339(datetime_str).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_date_int_basic() {
        let date = umt_new_date_int(2025, 1, 1, Some(0), Some(0), Some(0), Some(0));
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2025-01-01");
    }

    #[test]
    fn test_new_date_int_with_time() {
        let date = umt_new_date_int(2025, 1, 1, Some(10), Some(30), Some(45), Some(500));
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y").to_string(), "2025");
        assert_eq!(dt.format("%m").to_string(), "01");
        assert_eq!(dt.format("%d").to_string(), "01");
        assert_eq!(dt.format("%H").to_string(), "10");
        assert_eq!(dt.format("%M").to_string(), "30");
        assert_eq!(dt.format("%S").to_string(), "45");
    }

    #[test]
    fn test_new_date_int_full() {
        let date = umt_new_date_int_full(2025, 1, 1, 10, 30, 45, 500);
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(
            dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
            "2025-01-01 10:30:45.500"
        );
    }

    #[test]
    fn test_new_date_string_basic() {
        let date = umt_new_date_string("2025-01-01", None, None, None, None, None);
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y-%m-%d").to_string(), "2025-01-01");
    }

    #[test]
    fn test_new_date_string_with_time_and_tz() {
        let date = umt_new_date_string(
            "2025-01-01",
            Some("10"),
            Some("30"),
            Some("45"),
            Some("500"),
            Some("09"),
        );
        assert!(date.is_some());
        let dt = date.unwrap();
        // Check that UTC conversion is correct (10:30 JST = 01:30 UTC)
        assert_eq!(dt.format("%H:%M:%S").to_string(), "10:30:45");
    }

    #[test]
    fn test_new_date_string_timezone_difference() {
        let date_utc = umt_new_date_string("2025-01-01", Some("12"), Some("00"), None, None, Some("00"));
        let date_jst = umt_new_date_string("2025-01-01", Some("12"), Some("00"), None, None, Some("09"));

        assert!(date_utc.is_some());
        assert!(date_jst.is_some());

        let utc = date_utc.unwrap();
        let jst = date_jst.unwrap();

        // JST should be 9 hours behind UTC in terms of actual time
        assert_eq!(jst.timestamp() - utc.timestamp(), -9 * 3600);
    }

    #[test]
    fn test_new_date_from_iso() {
        let date = umt_new_date_from_iso("2025-01-01T10:30:45+09:00");
        assert!(date.is_some());
        let dt = date.unwrap();
        assert_eq!(dt.format("%Y-%m-%d %H:%M:%S").to_string(), "2025-01-01 10:30:45");
    }

    #[test]
    fn test_invalid_date() {
        let date = umt_new_date_int(2025, 2, 30, None, None, None, None); // Feb 30 doesn't exist
        assert!(date.is_none());
    }

    #[test]
    fn test_invalid_date_string() {
        let date = umt_new_date_string("invalid-date", None, None, None, None, None);
        assert!(date.is_none());
    }
}
