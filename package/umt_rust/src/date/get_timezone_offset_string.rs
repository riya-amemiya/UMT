//! Timezone offset string utilities.

use chrono::{DateTime, Local, TimeZone};

/// Gets timezone offset string in format "+HH:mm" or "-HH:mm".
///
/// # Arguments
///
/// * `offset_minutes` - The timezone offset in minutes (negative for west of UTC).
///                      Note: This follows JavaScript convention where getTimezoneOffset()
///                      returns the NEGATIVE of the UTC offset.
///
/// # Returns
///
/// A string representing the timezone offset (e.g., "+09:00" for JST).
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_timezone_offset_string;
///
/// // UTC+8 (JavaScript returns -480 for getTimezoneOffset())
/// assert_eq!(umt_get_timezone_offset_string(-480), "+08:00");
///
/// // UTC-5 (JavaScript returns 300 for getTimezoneOffset())
/// assert_eq!(umt_get_timezone_offset_string(300), "-05:00");
///
/// // UTC (JavaScript returns 0 for getTimezoneOffset())
/// assert_eq!(umt_get_timezone_offset_string(0), "+00:00");
/// ```
#[inline]
pub fn umt_get_timezone_offset_string(offset_minutes: i32) -> String {
    // JavaScript's getTimezoneOffset() returns minutes to ADD to get UTC
    // So we negate it to get the actual UTC offset
    let neg_minutes = -offset_minutes;
    let minutes = neg_minutes.abs();
    let hour_offset = minutes / 60;
    let minute_offset = minutes % 60;
    let sign = if neg_minutes >= 0 { '+' } else { '-' };

    format!("{}{:02}:{:02}", sign, hour_offset, minute_offset)
}

/// Gets timezone offset string for the local timezone.
///
/// # Returns
///
/// A string representing the local timezone offset (e.g., "+09:00" for JST).
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_local_timezone_offset_string;
///
/// // Returns something like "+09:00" depending on local timezone
/// let offset = umt_get_local_timezone_offset_string();
/// assert!(offset.starts_with('+') || offset.starts_with('-'));
/// ```
pub fn umt_get_local_timezone_offset_string() -> String {
    let local: DateTime<Local> = Local::now();
    let offset = local.offset();
    let total_seconds = offset.local_minus_utc();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds.abs() % 3600) / 60;
    let sign = if total_seconds >= 0 { '+' } else { '-' };

    format!("{}{:02}:{:02}", sign, hours.abs(), minutes)
}

/// Gets timezone offset string from a chrono DateTime.
///
/// # Arguments
///
/// * `dt` - A chrono DateTime with timezone info.
///
/// # Returns
///
/// A string representing the timezone offset (e.g., "+09:00" for JST).
///
/// # Examples
///
/// ```
/// use chrono::{FixedOffset, TimeZone};
/// use umt_rust::date::umt_get_timezone_offset_string_from_datetime;
///
/// let jst = FixedOffset::east_opt(9 * 3600).unwrap();
/// let dt = jst.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
/// assert_eq!(umt_get_timezone_offset_string_from_datetime(&dt), "+09:00");
/// ```
pub fn umt_get_timezone_offset_string_from_datetime<Tz: TimeZone>(dt: &DateTime<Tz>) -> String {
    let offset = dt.offset();
    let total_seconds = offset.fix().local_minus_utc();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds.abs() % 3600) / 60;
    let sign = if total_seconds >= 0 { '+' } else { '-' };

    format!("{}{:02}:{:02}", sign, hours.abs(), minutes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::FixedOffset;

    #[test]
    fn test_positive_timezone_offset() {
        // UTC+8 (JavaScript getTimezoneOffset returns -480)
        assert_eq!(umt_get_timezone_offset_string(-480), "+08:00");
    }

    #[test]
    fn test_negative_timezone_offset() {
        // UTC-5 (JavaScript getTimezoneOffset returns 300)
        assert_eq!(umt_get_timezone_offset_string(300), "-05:00");
    }

    #[test]
    fn test_zero_timezone_offset() {
        // UTC (JavaScript getTimezoneOffset returns 0)
        assert_eq!(umt_get_timezone_offset_string(0), "+00:00");
    }

    #[test]
    fn test_jst_offset() {
        // UTC+9 (JavaScript getTimezoneOffset returns -540)
        assert_eq!(umt_get_timezone_offset_string(-540), "+09:00");
    }

    #[test]
    fn test_half_hour_offset() {
        // UTC+5:30 (India Standard Time, JavaScript returns -330)
        assert_eq!(umt_get_timezone_offset_string(-330), "+05:30");
    }

    #[test]
    fn test_negative_half_hour_offset() {
        // UTC-3:30 (Newfoundland, JavaScript returns 210)
        assert_eq!(umt_get_timezone_offset_string(210), "-03:30");
    }

    #[test]
    fn test_from_datetime_jst() {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = jst.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        assert_eq!(umt_get_timezone_offset_string_from_datetime(&dt), "+09:00");
    }

    #[test]
    fn test_from_datetime_utc() {
        let utc = FixedOffset::east_opt(0).unwrap();
        let dt = utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        assert_eq!(umt_get_timezone_offset_string_from_datetime(&dt), "+00:00");
    }

    #[test]
    fn test_from_datetime_negative() {
        let est = FixedOffset::west_opt(5 * 3600).unwrap();
        let dt = est.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        assert_eq!(umt_get_timezone_offset_string_from_datetime(&dt), "-05:00");
    }

    #[test]
    fn test_local_timezone_offset_format() {
        let offset = umt_get_local_timezone_offset_string();
        // Should match format like "+09:00" or "-05:00"
        assert!(offset.len() == 6);
        assert!(offset.starts_with('+') || offset.starts_with('-'));
        assert_eq!(offset.chars().nth(3), Some(':'));
    }
}
