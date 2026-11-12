//! Date formatting utility.
//!
//! This module provides a function to format dates according to specified patterns.

use chrono::{DateTime, Datelike, Timelike, Utc};
use regex::Regex;
use std::fmt::Write;
use std::sync::LazyLock;

use super::get_timezone_offset_string::{
    umt_get_timezone_offset_string, umt_get_timezone_offset_string_compact,
};

static FORMAT_TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)]|YYYY|YY|MM|M|DD|D|d|HH|H|hh|h|mm|m|ss|s|SSS|A|a|ZZ|Z").unwrap()
});

/// Converts a date to a string according to the specified format pattern.
///
/// # Arguments
///
/// * `date` - The DateTime object to format
/// * `format_string` - The format pattern string (default: "YYYY-MM-DDTHH:mm:ssZ")
/// * `timezone_offset_minutes` - Timezone offset in minutes (default: 0 for UTC)
///
/// # Returns
///
/// The formatted date string
///
/// # Format Tokens
///
/// - `YYYY`: Full year (e.g., 2025)
/// - `YY`: Short year (e.g., 25)
/// - `MM`: Month with leading zero (01-12)
/// - `M`: Month without leading zero (1-12)
/// - `DD`: Day with leading zero (01-31)
/// - `D`: Day without leading zero (1-31)
/// - `d`: Day of week (0-6)
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
/// - `[text]`: Escaped text (not replaced)
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_format;
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 0).unwrap();
/// assert_eq!(umt_format(&date, "YYYY-MM-DD", 0), "2025-04-04");
/// assert_eq!(umt_format(&date, "HH:mm", 0), "15:30");
/// assert_eq!(umt_format(&date, "MM/DD/YYYY", 0), "04/04/2025");
/// ```
pub fn umt_format(
    date: &DateTime<Utc>,
    format_string: &str,
    timezone_offset_minutes: i32,
) -> String {
    let hours = date.hour();
    let hour_12 = if hours.is_multiple_of(12) {
        12
    } else {
        hours % 12
    };
    let mut result = String::with_capacity(format_string.len() + 16);
    let mut last_end = 0;

    for captures in FORMAT_TOKEN_RE.captures_iter(format_string) {
        let token = captures.get(0).unwrap();
        result.push_str(&format_string[last_end..token.start()]);
        last_end = token.end();

        if let Some(escaped) = captures.get(1) {
            result.push_str(escaped.as_str());
            continue;
        }

        let _ = match token.as_str() {
            "YYYY" => write!(result, "{:04}", date.year()),
            "YY" => {
                let year_string = date.year().to_string();
                result.push_str(&year_string[year_string.len().saturating_sub(2)..]);
                Ok(())
            }
            "MM" => write!(result, "{:02}", date.month()),
            "M" => write!(result, "{}", date.month()),
            "DD" => write!(result, "{:02}", date.day()),
            "D" => write!(result, "{}", date.day()),
            "d" => write!(result, "{}", date.weekday().num_days_from_sunday()),
            "HH" => write!(result, "{:02}", hours),
            "H" => write!(result, "{}", hours),
            "hh" => write!(result, "{:02}", hour_12),
            "h" => write!(result, "{}", hour_12),
            "mm" => write!(result, "{:02}", date.minute()),
            "m" => write!(result, "{}", date.minute()),
            "ss" => write!(result, "{:02}", date.second()),
            "s" => write!(result, "{}", date.second()),
            "SSS" => write!(result, "{:03}", date.nanosecond() / 1_000_000),
            "A" => write!(result, "{}", if hours < 12 { "AM" } else { "PM" }),
            "a" => write!(result, "{}", if hours < 12 { "am" } else { "pm" }),
            "ZZ" => write!(
                result,
                "{}",
                umt_get_timezone_offset_string_compact(timezone_offset_minutes)
            ),
            _ => write!(
                result,
                "{}",
                umt_get_timezone_offset_string(timezone_offset_minutes)
            ),
        };
    }

    result.push_str(&format_string[last_end..]);
    result
}

/// Format a date with the default ISO 8601 format.
///
/// # Arguments
///
/// * `date` - The DateTime object to format
/// * `timezone_offset_minutes` - Timezone offset in minutes (default: 0 for UTC)
///
/// # Returns
///
/// The formatted date string in ISO 8601 format
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_format_iso;
///
/// let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 45).unwrap();
/// assert_eq!(umt_format_iso(&date, 0), "2025-04-04T15:30:45+00:00");
/// ```
#[inline]
pub fn umt_format_iso(date: &DateTime<Utc>, timezone_offset_minutes: i32) -> String {
    umt_format(date, "YYYY-MM-DDTHH:mm:ssZ", timezone_offset_minutes)
}
