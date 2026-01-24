//! Date formatting utilities.

use chrono::{DateTime, Datelike, FixedOffset, NaiveDateTime, Offset, TimeZone, Timelike};
use regex::Regex;

/// Formats a DateTime according to the specified format pattern.
///
/// # Format Tokens
///
/// - `YYYY`: Full year (e.g., 2025)
/// - `YY`: Short year (e.g., 25)
/// - `MM`: Month with leading zero (01-12)
/// - `M`: Month without leading zero (1-12)
/// - `DD`: Day with leading zero (01-31)
/// - `D`: Day without leading zero (1-31)
/// - `d`: Day of week (0-6, where 0 is Sunday)
/// - `HH`: Hours with leading zero (00-23)
/// - `H`: Hours without leading zero (0-23)
/// - `hh`: Hours (12-hour) with leading zero (01-12)
/// - `h`: Hours (12-hour) without leading zero (1-12)
/// - `mm`: Minutes with leading zero (00-59)
/// - `m`: Minutes without leading zero (0-59)
/// - `ss`: Seconds with leading zero (00-59)
/// - `s`: Seconds without leading zero (0-59)
/// - `SSS`: Milliseconds with leading zeros (000-999)
/// - `A`: AM/PM
/// - `a`: am/pm
/// - `Z`: Timezone offset (+09:00)
/// - `ZZ`: Timezone offset without colon (+0900)
///
/// Text in square brackets `[...]` is treated as literal text.
///
/// # Arguments
///
/// * `dt` - The DateTime to format.
/// * `format_string` - The format pattern string (default: "YYYY-MM-DDTHH:mm:ssZ").
///
/// # Returns
///
/// The formatted date string.
///
/// # Examples
///
/// ```
/// use chrono::{FixedOffset, TimeZone};
/// use umt_rust::date::umt_format;
///
/// let jst = FixedOffset::east_opt(9 * 3600).unwrap();
/// let dt = jst.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
///
/// assert_eq!(umt_format(&dt, "YYYY-MM-DD"), "2023-06-10");
/// assert_eq!(umt_format(&dt, "YYYY/MM/DD HH:mm:ss"), "2023/06/10 15:30:45");
/// ```
pub fn umt_format<Tz: TimeZone>(dt: &DateTime<Tz>, format_string: &str) -> String {
    let year = dt.year();
    let month = dt.month();
    let day = dt.day();
    let hour = dt.hour();
    let minute = dt.minute();
    let second = dt.second();
    let millisecond = dt.timestamp_subsec_millis();
    let weekday = dt.weekday().num_days_from_sunday();

    let offset = dt.offset().fix();
    let offset_seconds = offset.local_minus_utc();
    let offset_hours = offset_seconds.abs() / 3600;
    let offset_minutes = (offset_seconds.abs() % 3600) / 60;
    let offset_sign = if offset_seconds >= 0 { '+' } else { '-' };
    let timezone_string = format!("{}{:02}:{:02}", offset_sign, offset_hours, offset_minutes);
    let timezone_string_no_colon = format!("{}{:02}{:02}", offset_sign, offset_hours, offset_minutes);

    let ampm = if hour < 12 { "AM" } else { "PM" };
    let hour_12 = {
        let h = hour % 12;
        if h == 0 { 12 } else { h }
    };

    // Build replacement map
    let replacements: Vec<(&str, String)> = vec![
        ("YYYY", format!("{:04}", year)),
        ("YY", format!("{:02}", year % 100)),
        ("MM", format!("{:02}", month)),
        ("M", month.to_string()),
        ("DD", format!("{:02}", day)),
        ("D", day.to_string()),
        ("d", weekday.to_string()),
        ("HH", format!("{:02}", hour)),
        ("H", hour.to_string()),
        ("hh", format!("{:02}", hour_12)),
        ("h", hour_12.to_string()),
        ("mm", format!("{:02}", minute)),
        ("m", minute.to_string()),
        ("ss", format!("{:02}", second)),
        ("s", second.to_string()),
        ("SSS", format!("{:03}", millisecond)),
        ("A", ampm.to_string()),
        ("a", ampm.to_lowercase()),
        ("ZZ", timezone_string_no_colon),
        ("Z", timezone_string),
    ];

    // Use regex to handle escaped text and format tokens
    let pattern =
        Regex::new(r"\[([^\]]+)]|(Y{1,4}|M{1,2}|D{1,2}|d{1,2}|H{1,2}|h{1,2}|a|A|m{1,2}|s{1,2}|Z{1,2}|SSS)")
            .unwrap();

    let result = pattern.replace_all(format_string, |caps: &regex::Captures| {
        // Check for escaped text first (group 1)
        if let Some(escaped) = caps.get(1) {
            return escaped.as_str().to_string();
        }

        // Handle format token (group 2)
        if let Some(token) = caps.get(2) {
            let token_str = token.as_str();
            for (key, value) in &replacements {
                if *key == token_str {
                    return value.clone();
                }
            }
        }

        // Shouldn't reach here, but return empty string as fallback
        String::new()
    });

    result.to_string()
}

/// Formats a DateTime with the default format "YYYY-MM-DDTHH:mm:ssZ".
///
/// # Arguments
///
/// * `dt` - The DateTime to format.
///
/// # Returns
///
/// The formatted date string in ISO 8601-like format.
///
/// # Examples
///
/// ```
/// use chrono::{FixedOffset, TimeZone};
/// use umt_rust::date::umt_format_default;
///
/// let jst = FixedOffset::east_opt(9 * 3600).unwrap();
/// let dt = jst.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
///
/// assert_eq!(umt_format_default(&dt), "2023-06-10T15:30:45+09:00");
/// ```
#[inline]
pub fn umt_format_default<Tz: TimeZone>(dt: &DateTime<Tz>) -> String {
    umt_format(dt, "YYYY-MM-DDTHH:mm:ssZ")
}

/// Formats a NaiveDateTime according to the specified format pattern.
///
/// Note: Timezone tokens (Z, ZZ) will use "+00:00" for NaiveDateTime.
///
/// # Arguments
///
/// * `dt` - The NaiveDateTime to format.
/// * `format_string` - The format pattern string.
///
/// # Returns
///
/// The formatted date string.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use umt_rust::date::umt_format_naive;
///
/// let dt = NaiveDate::from_ymd_opt(2023, 6, 10)
///     .unwrap()
///     .and_hms_milli_opt(15, 30, 45, 123)
///     .unwrap();
///
/// assert_eq!(umt_format_naive(&dt, "YYYY-MM-DD HH:mm:ss.SSS"), "2023-06-10 15:30:45.123");
/// ```
pub fn umt_format_naive(dt: &NaiveDateTime, format_string: &str) -> String {
    // Convert to DateTime with UTC offset for formatting
    let utc = FixedOffset::east_opt(0).unwrap();
    let dt_utc: DateTime<FixedOffset> = utc.from_utc_datetime(dt);
    umt_format(&dt_utc, format_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_datetime() -> DateTime<FixedOffset> {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        jst.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap()
    }

    fn create_test_datetime_with_ms() -> DateTime<FixedOffset> {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let naive = NaiveDate::from_ymd_opt(2023, 6, 10)
            .unwrap()
            .and_hms_milli_opt(15, 30, 45, 123)
            .unwrap();
        jst.from_local_datetime(&naive).single().unwrap()
    }

    #[test]
    fn test_format_basic_date() {
        let dt = create_test_datetime();
        assert_eq!(umt_format(&dt, "YYYY-MM-DD"), "2023-06-10");
    }

    #[test]
    fn test_format_date_time() {
        let dt = create_test_datetime();
        assert_eq!(umt_format(&dt, "YYYY/MM/DD HH:mm:ss"), "2023/06/10 15:30:45");
    }

    #[test]
    fn test_format_with_milliseconds() {
        let dt = create_test_datetime_with_ms();
        assert_eq!(
            umt_format(&dt, "YYYY-MM-DD HH:mm:ss.SSS"),
            "2023-06-10 15:30:45.123"
        );
    }

    #[test]
    fn test_format_with_timezone() {
        let dt = create_test_datetime_with_ms();
        assert_eq!(
            umt_format(&dt, "YYYY-MM-DD HH:mm:ss.SSS Z"),
            "2023-06-10 15:30:45.123 +09:00"
        );
    }

    #[test]
    fn test_format_with_timezone_no_colon() {
        let dt = create_test_datetime_with_ms();
        assert_eq!(
            umt_format(&dt, "YYYY-MM-DD HH:mm:ss.SSS ZZ"),
            "2023-06-10 15:30:45.123 +0900"
        );
    }

    #[test]
    fn test_format_default() {
        let dt = create_test_datetime();
        assert_eq!(umt_format_default(&dt), "2023-06-10T15:30:45+09:00");
    }

    #[test]
    fn test_format_escaped_characters() {
        let dt = create_test_datetime();
        assert_eq!(
            umt_format(&dt, "[Year:] YYYY [Month:] MM [Day:] DD"),
            "Year: 2023 Month: 06 Day: 10"
        );
    }

    #[test]
    fn test_format_short_tokens() {
        let dt = create_test_datetime();
        assert_eq!(umt_format(&dt, "YY-M-D"), "23-6-10");
    }

    #[test]
    fn test_format_12_hour_with_ampm() {
        let dt = create_test_datetime();
        assert_eq!(umt_format(&dt, "hh:mm:ss A"), "03:30:45 PM");
        assert_eq!(umt_format(&dt, "h:m:s a"), "3:30:45 pm");
    }

    #[test]
    fn test_format_day_of_week() {
        // June 10, 2023 was a Saturday (6)
        let dt = create_test_datetime();
        assert_eq!(umt_format(&dt, "d"), "6");
        assert_eq!(umt_format(&dt, "YYYY-MM-DD (d)"), "2023-06-10 (6)");
    }

    #[test]
    fn test_format_morning_hours() {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let naive = NaiveDate::from_ymd_opt(2023, 6, 10)
            .unwrap()
            .and_hms_milli_opt(9, 5, 8, 4)
            .unwrap();
        let dt = jst.from_local_datetime(&naive).single().unwrap();

        assert_eq!(umt_format(&dt, "HH:mm:ss"), "09:05:08");
        assert_eq!(umt_format(&dt, "H:m:s"), "9:5:8");
        assert_eq!(umt_format(&dt, "hh:mm A"), "09:05 AM");
        assert_eq!(umt_format(&dt, "h:mm a"), "9:05 am");
    }

    #[test]
    fn test_format_midnight() {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = jst.with_ymd_and_hms(2023, 6, 10, 0, 0, 0).unwrap();

        assert_eq!(umt_format(&dt, "HH:mm:ss"), "00:00:00");
        assert_eq!(umt_format(&dt, "hh:mm A"), "12:00 AM");
    }

    #[test]
    fn test_format_noon() {
        let jst = FixedOffset::east_opt(9 * 3600).unwrap();
        let dt = jst.with_ymd_and_hms(2023, 6, 10, 12, 0, 0).unwrap();

        assert_eq!(umt_format(&dt, "HH:mm:ss"), "12:00:00");
        assert_eq!(umt_format(&dt, "hh:mm A"), "12:00 PM");
    }

    #[test]
    fn test_format_naive() {
        let dt = NaiveDate::from_ymd_opt(2023, 6, 10)
            .unwrap()
            .and_hms_milli_opt(15, 30, 45, 123)
            .unwrap();

        assert_eq!(
            umt_format_naive(&dt, "YYYY-MM-DD HH:mm:ss.SSS"),
            "2023-06-10 15:30:45.123"
        );
    }

    #[test]
    fn test_format_negative_timezone() {
        let est = FixedOffset::west_opt(5 * 3600).unwrap();
        let dt = est.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();

        assert_eq!(umt_format(&dt, "Z"), "-05:00");
        assert_eq!(umt_format(&dt, "ZZ"), "-0500");
    }

    #[test]
    fn test_format_utc_timezone() {
        let utc = FixedOffset::east_opt(0).unwrap();
        let dt = utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();

        assert_eq!(umt_format(&dt, "Z"), "+00:00");
        assert_eq!(umt_format(&dt, "ZZ"), "+0000");
    }
}
