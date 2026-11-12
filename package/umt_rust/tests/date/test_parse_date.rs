//! Tests for the parse_date module.

use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};
use umt_rust::date::{umt_format, umt_parse_date};

fn date(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DateTime<Utc> {
    Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
        .unwrap()
}

#[test]
fn test_parses_date() {
    assert_eq!(
        umt_parse_date("2025-04-04", "YYYY-MM-DD"),
        Some(date(2025, 4, 4, 0, 0, 0))
    );
}

#[test]
fn test_parses_full_date_time_with_milliseconds() {
    assert_eq!(
        umt_parse_date("2023/06/10 15:30:45.123", "YYYY/MM/DD HH:mm:ss.SSS"),
        Some(date(2023, 6, 10, 15, 30, 45) + Duration::milliseconds(123))
    );
}

#[test]
fn test_parses_tokens_without_leading_zeros() {
    assert_eq!(
        umt_parse_date("23-6-1 5:7:9", "YY-M-D H:m:s"),
        Some(date(2023, 6, 1, 5, 7, 9))
    );
}

#[test]
fn test_reads_yy_as_2000_to_2099() {
    assert_eq!(umt_parse_date("99", "YY").unwrap().year(), 2099);
    assert_eq!(umt_parse_date("00", "YY").unwrap().year(), 2000);
}

#[test]
fn test_defaults_missing_fields() {
    assert_eq!(
        umt_parse_date("10:20", "HH:mm"),
        Some(date(1970, 1, 1, 10, 20, 0))
    );
    assert_eq!(umt_parse_date("", ""), Some(date(1970, 1, 1, 0, 0, 0)));
}

#[test]
fn test_keeps_years_below_100() {
    let parsed = umt_parse_date("0050-03-01", "YYYY-MM-DD").unwrap();
    assert_eq!((parsed.year(), parsed.month(), parsed.day()), (50, 3, 1));
}

#[test]
fn test_converts_12_hour_clock_with_meridiem() {
    assert_eq!(umt_parse_date("3:30 PM", "h:mm A").unwrap().hour(), 15);
    assert_eq!(umt_parse_date("03:30 am", "hh:mm a").unwrap().hour(), 3);
    assert_eq!(umt_parse_date("12:00 AM", "hh:mm A").unwrap().hour(), 0);
    assert_eq!(umt_parse_date("12:00 pm", "hh:mm a").unwrap().hour(), 12);
}

#[test]
fn test_uses_12_hour_values_as_written_without_meridiem() {
    assert_eq!(umt_parse_date("12", "hh").unwrap().hour(), 12);
    assert_eq!(umt_parse_date("7", "h").unwrap().hour(), 7);
}

#[test]
fn test_ignores_meridiem_for_24_hour_tokens() {
    assert_eq!(umt_parse_date("15 AM", "HH A").unwrap().hour(), 15);
}

#[test]
fn test_applies_z_offset() {
    assert_eq!(
        umt_parse_date("2025-04-04T15:30:00+09:00", "YYYY-MM-DDTHH:mm:ssZ"),
        Some(date(2025, 4, 4, 6, 30, 0))
    );
    assert_eq!(
        umt_parse_date("2025-04-04 01:15 -05:30", "YYYY-MM-DD HH:mm Z"),
        Some(date(2025, 4, 4, 6, 45, 0))
    );
}

#[test]
fn test_applies_zz_offset() {
    assert_eq!(
        umt_parse_date("2025-04-04 00:00 +0000", "YYYY-MM-DD HH:mm ZZ"),
        Some(date(2025, 4, 4, 0, 0, 0))
    );
    assert_eq!(
        umt_parse_date("2025-01-01 00:00 +0100", "YYYY-MM-DD HH:mm ZZ"),
        Some(date(2024, 12, 31, 23, 0, 0))
    );
}

#[test]
fn test_round_trips_with_format() {
    let pattern = "YYYY-MM-DD HH:mm:ss.SSS Z";
    let value = date(2023, 6, 10, 15, 30, 45) + Duration::milliseconds(123);
    assert_eq!(
        umt_parse_date(&umt_format(&value, pattern, 0), pattern),
        Some(value)
    );
}

#[test]
fn test_matches_escaped_and_literal_text() {
    assert_eq!(
        umt_parse_date("Year: 2023 (06)", "[Year:] YYYY (MM)"),
        Some(date(2023, 6, 1, 0, 0, 0))
    );
    assert_eq!(
        umt_parse_date("2023.06.10", "YYYY.MM.DD"),
        Some(date(2023, 6, 10, 0, 0, 0))
    );
    assert_eq!(umt_parse_date("2023x06x10", "YYYY.MM.DD"), None);
}

#[test]
fn test_accepts_matching_day_of_week() {
    assert_eq!(
        umt_parse_date("2025-04-21 1", "YYYY-MM-DD d"),
        Some(date(2025, 4, 21, 0, 0, 0))
    );
}

#[test]
fn test_rejects_mismatching_day_of_week() {
    assert_eq!(umt_parse_date("2025-04-21 2", "YYYY-MM-DD d"), None);
}

#[test]
fn test_last_occurrence_wins() {
    assert_eq!(
        umt_parse_date("2024 2025", "YYYY YYYY").unwrap().year(),
        2025
    );
}

#[test]
fn test_returns_none_when_string_does_not_match() {
    assert_eq!(umt_parse_date("2025-4-04", "YYYY-MM-DD"), None);
    assert_eq!(umt_parse_date("2025-04-04 extra", "YYYY-MM-DD"), None);
    assert_eq!(umt_parse_date("2025-04-04\n", "YYYY-MM-DD"), None);
    assert_eq!(umt_parse_date("abcd-04-04", "YYYY-MM-DD"), None);
    assert_eq!(umt_parse_date("10:00 XM", "hh:mm A"), None);
    assert_eq!(umt_parse_date("10:00 PM", "hh:mm a"), None);
}

#[test]
fn test_rejects_non_ascii_digits() {
    assert_eq!(
        umt_parse_date("\u{0662}\u{0660}\u{0662}\u{0665}", "YYYY"),
        None
    );
}

#[test]
fn test_returns_none_for_out_of_range_fields() {
    let cases = [
        ("2025-13-01", "YYYY-MM-DD"),
        ("2025-00-01", "YYYY-MM-DD"),
        ("2025-04-00", "YYYY-MM-DD"),
        ("2025-04-31", "YYYY-MM-DD"),
        ("2025-02-29", "YYYY-MM-DD"),
        ("24:00", "HH:mm"),
        ("10:60", "HH:mm"),
        ("10:00:60", "HH:mm:ss"),
        ("00 AM", "hh A"),
        ("13 PM", "hh A"),
        ("10:00 +24:00", "HH:mm Z"),
        ("10:00 +0960", "HH:mm ZZ"),
    ];
    for (string, pattern) in cases {
        assert_eq!(
            umt_parse_date(string, pattern),
            None,
            "{string} / {pattern}"
        );
    }
}

#[test]
fn test_accepts_february_29_in_leap_year() {
    assert_eq!(
        umt_parse_date("2024-02-29", "YYYY-MM-DD"),
        Some(date(2024, 2, 29, 0, 0, 0))
    );
}
