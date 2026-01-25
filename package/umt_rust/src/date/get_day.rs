//! Day name localization utility.
//!
//! This module provides a function to convert a day number to a localized day name.

/// Supported languages for day names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DayLanguage {
    /// German
    De,
    /// Korean
    Ko,
    /// English
    En,
    /// Japanese (default)
    #[default]
    Ja,
    /// French
    Fr,
}

const DAYS_DE: [&str; 7] = ["So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"];
const DAYS_KO: [&str; 7] = ["일", "월", "화", "수", "목", "금", "토"];
const DAYS_EN: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
const DAYS_JA: [&str; 7] = ["日", "月", "火", "水", "木", "金", "土"];
const DAYS_FR: [&str; 7] = ["Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"];

/// Convert a day number to a localized day name.
///
/// # Arguments
///
/// * `day` - Number representing the day (0-6, where 0 is Sunday)
/// * `lang` - Language code for the day name
///
/// # Returns
///
/// Day of the week string in the specified language
///
/// # Examples
///
/// ```
/// use umt_rust::date::{umt_get_day, DayLanguage};
///
/// assert_eq!(umt_get_day(0, DayLanguage::Ja), "日");   // Japanese Sunday
/// assert_eq!(umt_get_day(0, DayLanguage::En), "Sun");  // English Sunday
/// assert_eq!(umt_get_day(1, DayLanguage::Fr), "Lun");  // French Monday
/// assert_eq!(umt_get_day(1, DayLanguage::Ko), "월");   // Korean Monday
/// assert_eq!(umt_get_day(1, DayLanguage::De), "Mo");   // German Monday
/// ```
#[inline]
pub fn umt_get_day(day: u32, lang: DayLanguage) -> &'static str {
    let index = (day % 7) as usize;

    match lang {
        DayLanguage::De => DAYS_DE[index],
        DayLanguage::Ko => DAYS_KO[index],
        DayLanguage::En => DAYS_EN[index],
        DayLanguage::Ja => DAYS_JA[index],
        DayLanguage::Fr => DAYS_FR[index],
    }
}

/// Convert a day number to a Japanese day name.
///
/// Convenience function that calls `umt_get_day(day, DayLanguage::Ja)`.
///
/// # Arguments
///
/// * `day` - Number representing the day (0-6, where 0 is Sunday)
///
/// # Returns
///
/// Day of the week string in Japanese
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_day_ja;
///
/// assert_eq!(umt_get_day_ja(0), "日");
/// assert_eq!(umt_get_day_ja(1), "月");
/// ```
#[inline]
pub fn umt_get_day_ja(day: u32) -> &'static str {
    umt_get_day(day, DayLanguage::Ja)
}

/// Convert a day number to an English day name.
///
/// # Arguments
///
/// * `day` - Number representing the day (0-6, where 0 is Sunday)
///
/// # Returns
///
/// Day of the week string in English
///
/// # Examples
///
/// ```
/// use umt_rust::date::umt_get_day_en;
///
/// assert_eq!(umt_get_day_en(0), "Sun");
/// assert_eq!(umt_get_day_en(1), "Mon");
/// ```
#[inline]
pub fn umt_get_day_en(day: u32) -> &'static str {
    umt_get_day(day, DayLanguage::En)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_get_day_korean() {
        assert_eq!(umt_get_day(0, DayLanguage::Ko), "일");
        assert_eq!(umt_get_day(1, DayLanguage::Ko), "월");
        assert_eq!(umt_get_day(6, DayLanguage::Ko), "토");
    }

    #[test]
    fn test_get_day_german() {
        assert_eq!(umt_get_day(0, DayLanguage::De), "So");
        assert_eq!(umt_get_day(1, DayLanguage::De), "Mo");
        assert_eq!(umt_get_day(6, DayLanguage::De), "Sa");
    }

    #[test]
    fn test_get_day_french() {
        assert_eq!(umt_get_day(0, DayLanguage::Fr), "Dim");
        assert_eq!(umt_get_day(1, DayLanguage::Fr), "Lun");
        assert_eq!(umt_get_day(6, DayLanguage::Fr), "Sam");
    }

    #[test]
    fn test_get_day_overflow() {
        // Day 7 should wrap to day 0
        assert_eq!(umt_get_day(7, DayLanguage::En), "Sun");
        assert_eq!(umt_get_day(8, DayLanguage::En), "Mon");
    }

    #[test]
    fn test_convenience_functions() {
        assert_eq!(umt_get_day_ja(0), "日");
        assert_eq!(umt_get_day_en(0), "Sun");
    }
}
