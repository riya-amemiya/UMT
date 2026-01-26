//! Date formatting utility.
//!
//! This module provides a function to format dates according to specified patterns.

use chrono::{DateTime, Datelike, Timelike, Utc};
use regex::Regex;

use super::get_timezone_offset_string::{
    umt_get_timezone_offset_string, umt_get_timezone_offset_string_compact,
};

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
    let year_string = date.year().to_string();
    let month_string = date.month().to_string();
    let date_string = date.day().to_string();
    let hour_string = hours.to_string();
    let minute_string = date.minute().to_string();
    let second_string = date.second().to_string();
    let _millisecond_string = (date.nanosecond() / 1_000_000).to_string();
    let day_string = date.weekday().num_days_from_sunday().to_string();
    let ampm = if hours < 12 { "AM" } else { "PM" };

    let timezone_string = umt_get_timezone_offset_string(timezone_offset_minutes);
    let timezone_compact = umt_get_timezone_offset_string_compact(timezone_offset_minutes);

    let hour_12 = if hours.is_multiple_of(12) {
        12
    } else {
        hours % 12
    };

    // Build replacement map
    let replacements: Vec<(&str, String)> = vec![
        ("YYYY", format!("{:04}", date.year())),
        (
            "YY",
            year_string
                .chars()
                .skip(year_string.len().saturating_sub(2))
                .collect(),
        ),
        ("MM", format!("{:02}", date.month())),
        ("M", month_string),
        ("DD", format!("{:02}", date.day())),
        ("D", date_string),
        ("d", day_string),
        ("HH", format!("{:02}", hours)),
        ("H", hour_string),
        ("hh", format!("{:02}", hour_12)),
        ("h", hour_12.to_string()),
        ("mm", format!("{:02}", date.minute())),
        ("m", minute_string),
        ("ss", format!("{:02}", date.second())),
        ("s", second_string),
        ("SSS", format!("{:03}", date.nanosecond() / 1_000_000)),
        ("A", ampm.to_string()),
        ("a", ampm.to_lowercase()),
        ("ZZ", timezone_compact),
        ("Z", timezone_string),
    ];

    // Handle escaped text in brackets
    let re = Regex::new(r"\[([^\]]+)]").unwrap();
    let mut result = format_string.to_string();
    let mut escaped_texts: Vec<(String, String)> = Vec::new();

    // Replace escaped text with placeholders
    for (i, cap) in re.captures_iter(format_string).enumerate() {
        let placeholder = format!("\x00ESC{}\x00", i);
        let full_match = cap.get(0).unwrap().as_str();
        let inner_text = cap.get(1).unwrap().as_str();
        escaped_texts.push((placeholder.clone(), inner_text.to_string()));
        result = result.replace(full_match, &placeholder);
    }

    // Apply format replacements (order matters - longer patterns first)
    for (pattern, replacement) in &replacements {
        result = result.replace(pattern, replacement);
    }

    // Restore escaped texts
    for (placeholder, text) in escaped_texts {
        result = result.replace(&placeholder, &text);
    }

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
