//! Tests for the get_day module.

use umt_rust::date::{DayLanguage, umt_get_day, umt_get_day_en, umt_get_day_ja};

#[test]
fn test_get_day_sunday_in_different_languages() {
    assert_eq!(umt_get_day(0, DayLanguage::En), "Sun");
    assert_eq!(umt_get_day(0, DayLanguage::Ja), "日");
    assert_eq!(umt_get_day(0, DayLanguage::Ko), "일");
    assert_eq!(umt_get_day(0, DayLanguage::De), "So");
    assert_eq!(umt_get_day(0, DayLanguage::Fr), "Dim");
}

#[test]
fn test_get_day_weekdays_in_different_languages() {
    // Test Wednesday (3) in different languages
    assert_eq!(umt_get_day(3, DayLanguage::En), "Wed");
    assert_eq!(umt_get_day(3, DayLanguage::Ja), "水");
    assert_eq!(umt_get_day(3, DayLanguage::Ko), "수");
    assert_eq!(umt_get_day(3, DayLanguage::De), "Mi");
    assert_eq!(umt_get_day(3, DayLanguage::Fr), "Mer");
}

#[test]
fn test_get_day_japanese_default() {
    // Japanese as default through umt_get_day_ja
    assert_eq!(umt_get_day_ja(0), "日");
    assert_eq!(umt_get_day_ja(1), "月");
    assert_eq!(umt_get_day_ja(2), "火");
    assert_eq!(umt_get_day_ja(3), "水");
    assert_eq!(umt_get_day_ja(4), "木");
    assert_eq!(umt_get_day_ja(5), "金");
    assert_eq!(umt_get_day_ja(6), "土");
}

#[test]
fn test_get_day_invalid_day_numbers_wrap() {
    // Days should wrap around (using modulo)
    assert_eq!(umt_get_day(7, DayLanguage::Ja), "日"); // 7 % 7 = 0 (Sunday)
    assert_eq!(umt_get_day(8, DayLanguage::Ja), "月"); // 8 % 7 = 1 (Monday)
    assert_eq!(umt_get_day(100, DayLanguage::Ja), "火"); // 100 % 7 = 2 (Tuesday)
}

#[test]
fn test_get_day_all_days_in_english() {
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for (index, day) in days.iter().enumerate() {
        assert_eq!(umt_get_day(index as u32, DayLanguage::En), *day);
    }
}

#[test]
fn test_get_day_convenience_functions() {
    assert_eq!(umt_get_day_ja(0), "日");
    assert_eq!(umt_get_day_en(0), "Sun");
}

use umt_rust::date::*;

#[test]
fn test_convenience_functions() {
    assert_eq!(umt_get_day_ja(0), "日");
    assert_eq!(umt_get_day_en(0), "Sun");
}

#[test]
fn test_get_day_english() {
    assert_eq!(umt_get_day(0, DayLanguage::En), "Sun");
    assert_eq!(umt_get_day(1, DayLanguage::En), "Mon");
    assert_eq!(umt_get_day(2, DayLanguage::En), "Tue");
    assert_eq!(umt_get_day(3, DayLanguage::En), "Wed");
    assert_eq!(umt_get_day(4, DayLanguage::En), "Thu");
    assert_eq!(umt_get_day(5, DayLanguage::En), "Fri");
    assert_eq!(umt_get_day(6, DayLanguage::En), "Sat");
}

#[test]
fn test_get_day_french() {
    assert_eq!(umt_get_day(0, DayLanguage::Fr), "Dim");
    assert_eq!(umt_get_day(1, DayLanguage::Fr), "Lun");
    assert_eq!(umt_get_day(6, DayLanguage::Fr), "Sam");
}

#[test]
fn test_get_day_german() {
    assert_eq!(umt_get_day(0, DayLanguage::De), "So");
    assert_eq!(umt_get_day(1, DayLanguage::De), "Mo");
    assert_eq!(umt_get_day(6, DayLanguage::De), "Sa");
}

#[test]
fn test_get_day_japanese() {
    assert_eq!(umt_get_day(0, DayLanguage::Ja), "日");
    assert_eq!(umt_get_day(1, DayLanguage::Ja), "月");
    assert_eq!(umt_get_day(2, DayLanguage::Ja), "火");
    assert_eq!(umt_get_day(3, DayLanguage::Ja), "水");
    assert_eq!(umt_get_day(4, DayLanguage::Ja), "木");
    assert_eq!(umt_get_day(5, DayLanguage::Ja), "金");
    assert_eq!(umt_get_day(6, DayLanguage::Ja), "土");
}

#[test]
fn test_get_day_korean() {
    assert_eq!(umt_get_day(0, DayLanguage::Ko), "일");
    assert_eq!(umt_get_day(1, DayLanguage::Ko), "월");
    assert_eq!(umt_get_day(6, DayLanguage::Ko), "토");
}

#[test]
fn test_get_day_overflow() {
    // Day 7 should wrap to day 0
    assert_eq!(umt_get_day(7, DayLanguage::En), "Sun");
    assert_eq!(umt_get_day(8, DayLanguage::En), "Mon");
}
