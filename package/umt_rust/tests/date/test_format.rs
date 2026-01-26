//! Tests for the format module.

use chrono::{TimeZone, Utc};
use umt_rust::date::{umt_format, umt_format_iso, umt_get_timezone_offset_string};

#[test]
fn test_format_date_correctly() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
    assert_eq!(umt_format(&date, "YYYY-MM-DD", 0), "2023-06-10");
    assert_eq!(
        umt_format(&date, "YYYY/MM/DD HH:mm:ss", 0),
        "2023/06/10 15:30:45"
    );
}

#[test]
fn test_format_with_milliseconds() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
    // Note: chrono's with_ymd_and_hms doesn't set milliseconds, so SSS will be 000
    assert_eq!(
        umt_format(&date, "YYYY-MM-DD HH:mm:ss.SSS", 0),
        "2023-06-10 15:30:45.000"
    );
}

#[test]
fn test_format_with_timezone() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
    let expected_with_tz = format!(
        "2023-06-10 15:30:45.000 {}",
        umt_get_timezone_offset_string(0)
    );
    assert_eq!(
        umt_format(&date, "YYYY-MM-DD HH:mm:ss.SSS Z", 0),
        expected_with_tz
    );

    let expected_with_tz_compact = format!(
        "2023-06-10 15:30:45.000 {}",
        umt_get_timezone_offset_string(0).replace(":", "")
    );
    assert_eq!(
        umt_format(&date, "YYYY-MM-DD HH:mm:ss.SSS ZZ", 0),
        expected_with_tz_compact
    );
}

#[test]
fn test_format_default_iso() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
    let expected = format!("2023-06-10T15:30:45{}", umt_get_timezone_offset_string(0));
    assert_eq!(umt_format_iso(&date, 0), expected);
}

#[test]
fn test_format_escaped_characters() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
    assert_eq!(
        umt_format(&date, "[Year:] YYYY [Month:] MM [Day:] DD", 0),
        "Year: 2023 Month: 06 Day: 10"
    );
}

#[test]
fn test_format_different_formats() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 15, 30, 45).unwrap();
    assert_eq!(umt_format(&date, "YY-M-D", 0), "23-6-10");
    assert_eq!(umt_format(&date, "hh:mm:ss A", 0), "03:30:45 PM");
    assert_eq!(umt_format(&date, "h:m:s a", 0), "3:30:45 pm");
}

#[test]
fn test_format_day_of_week() {
    // June 10, 2023 was a Saturday (day 6)
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 0, 0, 0).unwrap();
    assert_eq!(umt_format(&date, "d", 0), "6");
    assert_eq!(umt_format(&date, "YYYY-MM-DD (d)", 0), "2023-06-10 (6)");
}

#[test]
fn test_format_morning_hours() {
    let date = Utc.with_ymd_and_hms(2023, 6, 10, 9, 5, 8).unwrap();
    assert_eq!(umt_format(&date, "HH:mm:ss", 0), "09:05:08");
    assert_eq!(umt_format(&date, "H:m:s", 0), "9:5:8");
    assert_eq!(umt_format(&date, "hh:mm A", 0), "09:05 AM");
    assert_eq!(umt_format(&date, "h:mm a", 0), "9:05 am");
}

use umt_rust::date::*;

#[test]
fn test_format_12_hour() {
    let date_am = Utc.with_ymd_and_hms(2025, 4, 4, 9, 30, 0).unwrap();
    let date_pm = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 0).unwrap();
    let date_noon = Utc.with_ymd_and_hms(2025, 4, 4, 12, 0, 0).unwrap();
    let date_midnight = Utc.with_ymd_and_hms(2025, 4, 4, 0, 0, 0).unwrap();

    assert_eq!(umt_format(&date_am, "h:mm A", 0), "9:30 AM");
    assert_eq!(umt_format(&date_pm, "h:mm A", 0), "3:30 PM");
    assert_eq!(umt_format(&date_noon, "h:mm A", 0), "12:00 PM");
    assert_eq!(umt_format(&date_midnight, "h:mm A", 0), "12:00 AM");
}

#[test]
fn test_format_date_only() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 0, 0, 0).unwrap();
    assert_eq!(umt_format(&date, "YYYY-MM-DD", 0), "2025-04-04");
}

#[test]
fn test_format_escaped_text() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 0).unwrap();
    assert_eq!(
        umt_format(&date, "[Date:] YYYY-MM-DD", 0),
        "Date: 2025-04-04"
    );
}

#[test]
fn test_format_iso() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 45).unwrap();
    assert_eq!(umt_format_iso(&date, 0), "2025-04-04T15:30:45+00:00");
    assert_eq!(umt_format_iso(&date, 540), "2025-04-04T15:30:45+09:00");
}

#[test]
fn test_format_lowercase_ampm() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 0).unwrap();
    assert_eq!(umt_format(&date, "h:mm a", 0), "3:30 pm");
}

#[test]
fn test_format_milliseconds() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 45).unwrap();
    // Note: with_ymd_and_hms doesn't set milliseconds, so SSS will be 000
    assert_eq!(umt_format(&date, "HH:mm:ss.SSS", 0), "15:30:45.000");
}

#[test]
fn test_format_no_padding() {
    let date = Utc.with_ymd_and_hms(2025, 1, 5, 9, 5, 3).unwrap();
    assert_eq!(umt_format(&date, "M/D/YYYY H:m:s", 0), "1/5/2025 9:5:3");
}

#[test]
fn test_format_short_year() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 0, 0, 0).unwrap();
    assert_eq!(umt_format(&date, "YY-MM-DD", 0), "25-04-04");
}

#[test]
fn test_format_time_only() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 45).unwrap();
    assert_eq!(umt_format(&date, "HH:mm:ss", 0), "15:30:45");
}

#[test]
fn test_format_timezone() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 0, 0, 0).unwrap();
    assert_eq!(umt_format(&date, "Z", 540), "+09:00");
    assert_eq!(umt_format(&date, "ZZ", 540), "+0900");
    assert_eq!(umt_format(&date, "Z", -300), "-05:00");
}

#[test]
fn test_format_us_style() {
    let date = Utc.with_ymd_and_hms(2025, 4, 4, 0, 0, 0).unwrap();
    assert_eq!(umt_format(&date, "MM/DD/YYYY", 0), "04/04/2025");
}
